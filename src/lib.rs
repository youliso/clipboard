#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use clipboard_rs::{Clipboard, ClipboardContext, ContentFormat};

#[napi]
pub fn get_files() -> Vec<String> {
  let ctx = ClipboardContext::new().unwrap();
  if ctx.has(ContentFormat::Files) {
    let files = ctx.get_files();
    match files {
      Ok(files) => files,
      Err(err) => {
        eprintln!("Failed to get files from clipboard: {}", err);
        vec![]
      }
    }
  } else {
    vec![]
  }
}

#[napi]
pub fn set_files(files: Vec<String>) -> bool {
  let ctx = ClipboardContext::new().unwrap();
  match ctx.set_files(files) {
    Ok(_) => true,
    Err(err) => {
      eprintln!("Failed to set files to clipboard: {}", err);
      false
    }
  }
}
