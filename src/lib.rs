#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use clipboard_rs::{Clipboard, ClipboardContext, ContentFormat};
use napi::Error;

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
pub fn set_files(files: Vec<String>) -> Result<bool, Error> {
  // 基本输入验证
  if files.is_empty() {
    return Err(Error::from_reason(format!("Failed to files empty")));
  }

  // 创建 ClipboardContext 并处理可能的错误
  let ctx = ClipboardContext::new()
    .map_err(|err| Error::from_reason(format!("Failed to create ClipboardContext: {}", err)))?;

  // 设置文件到剪贴板并处理可能的错误
  match ctx.set_files(files) {
    Ok(_) => Ok(true),
    Err(err) => {
      eprintln!("Failed to set files to clipboard: {}", err);
      Err(Error::from_reason(format!(
        "Failed to set files to clipboard: {}",
        err
      )))
    }
  }
}
