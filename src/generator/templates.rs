use include_dir::{Dir, include_dir};
use std::{io, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct TemplateMetadata {
    pub name: String,
    pub language: Option<String>
}

pub struct TemplateInfo {
    pub id: String,
    pub metadata: TemplateMetadata,
}


pub(super) static TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates");

pub fn template_names() -> Vec<String> {
    let mut names = Vec::new();

    for dir in TEMPLATES.dirs() {
        if dir.get_dir(dir.path().join("files")).is_some() {
            let name = dir.path().to_string_lossy().into_owned();
            names.push(name);
        }
    }

    names.sort();

    names
}

pub fn template_metadata(id:&str) -> io::Result<TemplateMetadata> {
    let path = Path::new(id).join("template.toml");

    let file = TEMPLATES.get_file(&path);

    let file = match file {
        Some(file) => file,

        None => 
            return Err(io::Error::new(
                io::ErrorKind::NotFound, 
                "Не найден template.toml"
            ))
    };

    let text = file.contents_utf8();

    let text = match text {
        Some(text) => text,

       None => 
            return Err(io::Error::new(
                io::ErrorKind::InvalidData, 
                "template.toml содержит некорректный UTF-8"
            ))
    };

    let metadata = toml::from_str::<TemplateMetadata>(text);

    match metadata {
        Ok(value) => Ok(value),
        Err(error) => 
            Err(io::Error::new(
                io::ErrorKind::InvalidData, 
                error
            ))
    }
}

pub fn list_templates() -> io::Result<Vec<TemplateInfo>> {
    let mut templates = Vec::new();

    for id in template_names() {
        let metadata = template_metadata(&id)?;
        templates.push(TemplateInfo {
            id,
            metadata,
        });
    }
    Ok(templates)
}