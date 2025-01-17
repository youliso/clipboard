#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use clipboard_rs::{Clipboard, ClipboardContext, ContentFormat};

#[napi]
pub fn get_files() -> Vec<String> {
  match ClipboardContext::new() {
    Ok(ctx) => {
      if ctx.has(ContentFormat::Files) {
        match ctx.get_files() {
          Ok(files) => files,
          Err(err) => {
            eprintln!("Failed to get files from clipboard: {}", err);
            vec![]
          }
        }
      } else {
        println!("Clipboard does not contain files.");
        vec![]
      }
    }
    Err(err) => {
      eprintln!("Failed to create clipboard context: {}", err);
      vec![]
    }
  }
}

#[napi]
pub fn set_files(files: Vec<String>) -> bool {
  // 基本输入验证
  if files.is_empty() {
    return false;
  }

  match ClipboardContext::new() {
    Ok(ctx) => match ctx.set_files(files) {
      Ok(_) => true,
      Err(err) => {
        eprintln!("Failed to set files to clipboard: {}", err);
        false
      }
    },
    Err(err) => {
      eprintln!("Failed to create clipboard context: {}", err);
      false
    }
  }
}
