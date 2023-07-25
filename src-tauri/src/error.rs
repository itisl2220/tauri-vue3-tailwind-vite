use rodio::PlayError;
use scraper::error::SelectorErrorKind;
use serde::{Deserialize, Serialize};


pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
  pub msg: String,  // error message
  pub code: i32, // error code
  pub error: Option<String>, // optional error message
}


impl Error {
  pub fn new(msg: String, code: i32, error: Option<String>) -> Self {
    Self {
      msg,
      code,
      error,
    }
  }
}

impl From<rodio::decoder::DecoderError> for Error {
  fn from(e: rodio::decoder::DecoderError) -> Self {
    Self {
      msg: "DecoderError Error".to_string(),
      code: 1,
      error: Some(e.to_string()),
    }
  }
}

impl From<PlayError> for Error {
  fn from(e: PlayError) -> Self {
    Self {
      msg: "PlayError Error".to_string(),
      code: 1,
      error: Some(e.to_string()),
    }
  }
}

impl From<rodio::StreamError> for Error {
  fn from(e: rodio::StreamError) -> Self {
    Self {
      msg: "StreamError Error".to_string(),
      code: 1,
      error: Some(e.to_string()),
    }
  }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
  fn from(e: reqwest::header::InvalidHeaderValue) -> Self {
    Self {
      msg: "InvalidHeaderValue Error".to_string(),
      code: 1,
      error: Some(e.to_string()),
    }
  }
}



impl From<base64::DecodeError> for Error {
  fn from(e: base64::DecodeError) -> Self {
    Self {
      msg: "Base64 Error".to_string(),
      code: 1,
      error: Some(e.to_string()),
    }
  }
}

impl From<SelectorErrorKind<'_>> for Error {
  fn from(e: SelectorErrorKind) -> Self {
    Self {
      msg: "Selector Error".to_string(),
      code: 1,
      error: Some(e.to_string()),
    }
  }
}

impl From<std::io::Error> for Error {
  fn from(e: std::io::Error) -> Self {
    Self {
      msg: "TbApi Error".to_string(),
      code: 1,
      error: Some(e.to_string()),
    }
  }
}



impl From<webview2_com::Error> for Error {
  fn from(err: webview2_com::Error) -> Self {
    Self{
      msg: "webview2_com Error".to_string(),
      code: 1,
      error: Some(err.to_string()),
    }
  }
}

impl From<serde_json::Error> for Error {
  fn from(err: serde_json::Error) -> Self {
      Self {
        msg: "serde_json Error".to_string(),
        code: 1,
        error: Some(err.to_string()),
    }
  }
}

impl From<tauri::Error> for Error {
  fn from(err: tauri::Error) -> Self {
      Self {
        msg: "tauri Error".to_string(),
        code: 1,
        error: Some(err.to_string()),
      }
  }
}

impl From<reqwest::Error> for Error {
  fn from(err: reqwest::Error) -> Self {
      Self {
        msg: "reqwest Error".to_string(),
        code: 1,
        error: Some(err.to_string()),
      }
  }
}

impl From<sled::Error> for Error {
  fn from(err: sled::Error) -> Self {
      Self {
        msg: "sled Error".to_string(),
        code: 1,
        error: Some(err.to_string()),
      }
  }
}

// impl From<FromUtf8Error> for Error {
//   fn from(err: FromUtf8Error) -> Self {
//       Self {
//         msg: "FromUtf8Error Error".to_string(),
//         code: 1,
//         error: Some(err.to_string()),
//       }
//   }
// }
