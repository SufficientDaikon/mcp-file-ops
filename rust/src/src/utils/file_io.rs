use anyhow::Result;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub async fn read_file_lines(path: &str) -> Result<(Vec<String>, String, String)> {
    let content: String = fs::read_to_string(path).await?;

    let line_ending = if content.contains("\r\n") {
        "crlf".to_string()
    } else {
        "lf".to_string()
    };

    let lines: Vec<String> = content
        .lines()
        .map(|line: &str| line.to_string())
        .collect();

    Ok((lines, "utf-8".to_string(), line_ending))
}

pub async fn atomic_write(
    path: &str,
    lines: &[String],
    _encoding: &str,
    line_ending: &str,
) -> Result<()> {
    let separator = match line_ending {
        "crlf" => "\r\n",
        _ => "\n",
    };

    let content = lines.join(separator);
    let path_obj = Path::new(path);
    let _parent = path_obj.parent().unwrap_or_else(|| Path::new("."));

    // Create temp file
    let temp_path = format!("{}.tmp", path);
    let mut file: tokio::fs::File = fs::File::create(&temp_path).await?;
    file.write_all(content.as_bytes()).await?;
    file.sync_all().await?;
    drop(file);

    // Atomic rename
    fs::rename(&temp_path, path).await?;

    Ok(())
}
