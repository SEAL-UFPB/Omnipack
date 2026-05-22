use std::collections::HashMap;
use std::fs;
// linguagens suportadas atualmente = Go, Rust
struct DependencyManagers<'a> {
    dependency_managers: HashMap<&'a str, &'a str>,
}
impl<'a> DependencyManagers<'a> {
    fn new() -> Self {
        Self {
            // instanciando com capacidade 2 devido à quantidade de linguagens suportadas atualmente
            dependency_managers: HashMap::with_capacity(2),
        }
    }
}

// função que recebe uma str para indicar o caminho e lê os arquivos da pasta
// retorna Ok(lang) caso ache uma linguagem suportada
// retorna Err() caso não identifique a linguagem ou receba algum outro erro no caminho
fn language(path: &'static str) -> Result<&'static str, Box<dyn std::error::Error>> {
    let mut depends = DependencyManagers::new();
    depends.dependency_managers.insert("cargo.toml", "rust");
    depends.dependency_managers.insert("go.mod", "go");
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
    println!("{:?}", language("."))
}
