#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::io::prelude::*;
use zip::write::FileOptions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ZipContent {
  directories: Vec<String>,
  filenames: Vec<String>,
  contents: Vec<String>,
}

#[tauri::command]
fn generate_epub(filename: String, content: String) -> Result<String, String> {
  match create_zip(&filename, &content) {
    Ok(_) => Ok("ok".into()),
    Err(e) => Err(e.to_string()),
  }
}

fn create_zip(filename: &str, content: &str) -> zip::result::ZipResult<()> {
  let v: ZipContent = serde_json::from_str(content).unwrap();

  let path = std::path::Path::new(filename);
  let file = std::fs::File::create(&path).unwrap();

  let mut zip = zip::ZipWriter::new(file);

  // Create directories
  for dir in v.directories {
      zip.add_directory(dir, Default::default())?;
  }

  let options = FileOptions::default()
      .compression_method(zip::CompressionMethod::Stored)
      .unix_permissions(0o755);

  for index in 0..v.filenames.len() {
      zip.start_file(&v.filenames[index], options)?;
      zip.write_all(v.contents[index].as_bytes())?;
  }

  zip.finish()?;

  Ok(())
}

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![generate_epub])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
