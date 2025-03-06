use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(about, version)]
struct Args {
    save_path: PathBuf,

    #[command(flatten)]
    article: Article,
}

#[derive(serde::Serialize, serde::Deserialize, clap::Args)]
struct Article {
    title: String,
    body: String,
}

fn main() {
    let Args { save_path, article } = Args::parse();

    println!("Save path: {:?}", save_path);

    let content = match save_path.extension() {
        Some(ext) if ext == "json" => serde_json::to_string(&article).unwrap(),
        Some(ext) if ext == "toml" => toml::to_string(&article).unwrap(),
        _ => {
            eprintln!("Unsupported file extension");
            return;
        }
    };

    println!("Content: \n===\n{}\n===\n", content);

    std::fs::write(save_path, content).unwrap();
}
