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
            dependency_managers: HashMap::with_capacity(4),
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
    depends.dependency_managers.insert("pyproject.toml", "python");
    depends.dependency_managers.insert("CMakeLists.txt", "c++");
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
    if lang.is_empty() {
        Err("language not identified".into())
    } else {
        Ok(lang)
    }
}

// Tenta da o match numa linguagem suportada e devolve o cmd pra compilar o projeto
fn get_build_command(lang: &str) -> &'static str 
{
    match lang
    {
        "rust" => "cargo build --release",
        "go" => "go build -v",
        "python" => "python -m venv venv",
        "c++" => "cmake --build build --config Release",
        _ => "echo 'Comando de build não foi encontrado'",
    }
}

fn main() {
    match language(".") 
    {
        Ok(lang) => {
            println!("Linguagem foi identificada: {}", lang);
            println!("Comando de compilação sugerido: {}", get_build_command(lang));
        },
        Err(e) => println!("Erro ao detectar: {}", e),
    }
}
