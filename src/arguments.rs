use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "Calculator de rambursari anticipate")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub(crate) struct Arguments {
    #[arg(value_parser = check_if_path_exists)]
    pub(crate) input_file: PathBuf,
}

fn check_if_path_exists(path: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(path);
    if path.exists() {
        Ok(path)
    } else {
        Err(format!("Fișierul nu există : {}", path.display()))
    }
}