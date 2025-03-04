use anyhow::{Context, Result};
use clap::Parser;

// ファイル内のパターンを検索し、一致する行を表示するコマンドラインツール
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    // 検索するパターン
    pattern: String,
    // 検索するファイルのパス
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file: {:?}", args.path.display()))?;

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}
