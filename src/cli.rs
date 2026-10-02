use std::{io, path::PathBuf};

use dialoguer::console::{Style, style};
use dialoguer::{Input, Select, theme::ColorfulTheme};

pub struct NewArgs {
    pub template: String,
    pub destination: PathBuf,
}

pub fn parse() -> Result<NewArgs, std::io::Error> {

    let templates = crate::generator::list_templates()?;

    let mut languages = Vec::new();

    for template in &templates {
        languages.push(template.metadata.language.clone());
    }

    languages.sort();
    languages.dedup();

    let mut language_labels = Vec::new();

    language_labels.push(String::from("Все языки"));

    for language in &languages {
        let label = match language {
            Some(name) => name.clone(),
            None => String::from("Без привязки к языку")
        };

        language_labels.push(label);
    }


    if templates.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Нет доступных шаблонов",
        ));
    };

    let theme = ColorfulTheme {
        prompt_style: Style::new().bold(),
        prompt_prefix: style("?".to_owned()).cyan().bold(),
        active_item_style: Style::new().cyan().bold(),
        active_item_prefix: style("❯".to_owned()).cyan().bold(),
        values_style: Style::new().cyan(),
        success_prefix: style("✔".to_owned()).green(),
        hint_style: Style::new().dim(),
        ..ColorfulTheme::default()
    };

    let language_index = Select::with_theme(&theme)
        .with_prompt("Выберите язык")
        .items(&language_labels)
        .default(0)
        .interact()
        .map_err(std::io::Error::other)?;

    let mut filtered_templates = Vec::new();

    for template in &templates {
        if language_index == 0 {
            filtered_templates.push(template);
        } else if template.metadata.language == languages[language_index - 1] {
            filtered_templates.push(template);
        }
    }

    let mut names = Vec::new();

    for template in &filtered_templates {

        if language_index == 0 {
            let language = template.metadata.language.as_deref()
                .unwrap_or("Без привязки к языку");

            let label = format!("{} [{}]", template.metadata.name,language);
            names.push(label);
        } else {
            names.push(template.metadata.name.clone());
        }
    }

    let index = Select::with_theme(&theme)
        .with_prompt("Выберите шаблон")
        .items(&names)
        .default(0)
        .interact()
        .map_err(std::io::Error::other)?;

    let destination = Input::<String>::with_theme(&theme)
        .with_prompt("Название проекта")
        .interact_text()
        .map_err(io::Error::other)?;

    Ok(NewArgs {
        template: filtered_templates[index].id.clone(),
        destination: PathBuf::from(destination),
    })
}
