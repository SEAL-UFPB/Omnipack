use std::collections::HashMap;
use std::fs;
struct DependencyManagers<'a> {
    dependency_managers: HashMap<&'a str, &'a str>,
}
impl<'a> DependencyManagers<'a> {
    fn new() -> Self {
        Self {
            dependency_managers: HashMap::new(),
        }
    }
}
fn language() -> Result<&'static str, Box<dyn std::error::Error>> {
    let mut depends = DependencyManagers::new();
    depends.dependency_managers.insert("cargo.toml", "rust");
    depends.dependency_managers.insert("go.mod", "go");
    let path = ".";
    let dir = fs::read_dir(path)?;
    let mut lang: &str = "";
    for file in dir {
        let archive = file?;
        let archive = archive.file_name();
        let archive = archive.to_string_lossy();
        let archive = archive.trim().to_lowercase();
        if let Some(language) = depends.dependency_managers.get(archive.as_str()) {
            lang = language;
            break;
        } else {
            continue;
        }
    }
    if !lang.eq("") {
        Ok(lang)
    } else {
        Err("language not identified".into())
    }
}
fn main() {
    println!("{:?}", language())
}
