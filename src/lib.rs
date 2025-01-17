#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use clipboard_rs::{Clipboard, ClipboardContext, ContentFormat};
use napi::bindgen_prelude::Buffer;

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

#[napi]
pub fn get_buffer(format: String) -> Option<Buffer> {
  match ClipboardContext::new() {
    Ok(ctx) => match ctx.get_buffer(&format) {
      Ok(buf) => Some(Buffer::from(buf)),
      Err(err) => {
        eprintln!("Failed to get buffer from clipboard: {}", err);
        None
      }
    },
    Err(err) => {
      eprintln!("Failed to create clipboard context: {}", err);
      None
    }
  }
}

#[napi]
pub fn set_buffer(format: String, buf: Buffer) -> bool {
  let value_data: Vec<u8> = buf.into();
  match ClipboardContext::new() {
    Ok(ctx) => match ctx.set_buffer(&format, value_data) {
      Ok(_) => true,
      Err(err) => {
        eprintln!("Failed to set buffer from clipboard: {}", err);
        false
      }
    },
    Err(err) => {
      eprintln!("Failed to create clipboard context: {}", err);
      false
    }
  }
}

#[napi]
pub fn get_html() -> Option<String> {
  match ClipboardContext::new() {
    Ok(ctx) => match ctx.get_html() {
      Ok(text) => Some(text),
      Err(err) => {
        eprintln!("Failed to get html from clipboard: {}", err);
        None
      }
    },
    Err(err) => {
      eprintln!("Failed to create clipboard context: {}", err);
      None
    }
  }
}

#[napi]
pub fn set_html(html: String) -> bool {
  match ClipboardContext::new() {
    Ok(ctx) => match ctx.set_html(html) {
      Ok(_) => true,
      Err(err) => {
        eprintln!("Failed to set html from clipboard: {}", err);
        false
      }
    },
    Err(err) => {
      eprintln!("Failed to create clipboard context: {}", err);
      false
    }
  }
}

#[napi]
pub fn get_text() -> Option<String> {
  match ClipboardContext::new() {
    Ok(ctx) => match ctx.get_text() {
      Ok(text) => Some(text),
      Err(err) => {
        eprintln!("Failed to get text from clipboard: {}", err);
        None
      }
    },
    Err(err) => {
      eprintln!("Failed to create clipboard context: {}", err);
      None
    }
  }
}

#[napi]
pub fn set_text(text: String) -> bool {
  match ClipboardContext::new() {
    Ok(ctx) => match ctx.set_text(text) {
      Ok(_) => true,
      Err(err) => {
        eprintln!("Failed to set text from clipboard: {}", err);
        false
      }
    },
    Err(err) => {
      eprintln!("Failed to create clipboard context: {}", err);
      false
    }
  }
}
