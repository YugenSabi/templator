use std::{fs, io, path::Path};

use include_dir::Dir;


use super::templates::TEMPLATES;

pub fn run(template: &str, destination: &Path) -> io::Result<()> {

    let project_name = destination.file_name();

    let project_name = match project_name {
        Some(name) => name,
        None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Ошибка ввода"))
        
    };

    let project_name = match project_name.to_str() {
        Some(name) => name,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput, 
                "Имя проекта содержит некорректный UTF-8"
            ))
        }
    };

    let source = TEMPLATES
        .dirs()
        .find(|dir| dir.path() == Path::new(template))
        .and_then(|dir| dir.get_dir(Path::new(template).join("files")))
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Неизвестный шаблон"))?;

    copy_directory(source, destination,project_name)
}

fn copy_directory(source: &Dir<'_>, destination: &Path, project_name:&str) -> io::Result<()> {

    fs::create_dir(destination)?;

    for file in source.files() {
        let name = file.path().file_name().expect("У файла есть имя");

        match file.contents_utf8() {
            Some(text) => { 
                let content = text.replace("{project_name}", project_name);
                fs::write(destination.join(name), content)?;
            },
            None => {
                fs::write(destination.join(name), file.contents())?;
            }
        }
    }

    for dir in source.dirs() {
        let name = dir.path().file_name().expect("У вложенной папки есть имя");
        copy_directory(dir, &destination.join(name),project_name)?;
    }

    Ok(())
}
