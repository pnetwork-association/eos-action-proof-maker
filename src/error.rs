use hex;
use std::{
    fmt,
    fs::read_to_string,
};

#[derive(Debug)]
pub enum AppError {
    Custom(String),
    IOError(std::io::Error),
    HexError(hex::FromHexError),
    EosPrimitivesNamesError(eos_primitives::ParseNameError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            AppError::Custom(ref msg) =>
                format!("{}", msg),
            AppError::HexError(ref e) =>
                format!("✘ Hex Error!\n✘ {}", e),
            AppError::IOError(ref e) =>
                format!("✘ I/O Error!\n✘ {}", e),
            AppError::EosPrimitivesNamesError(ref e) =>
                format!("✘ Eos Primitives Names Error!\n✘ {:?}", e),
        };
        f.write_fmt(format_args!("{}", msg))
    }
}

impl From<hex::FromHexError> for AppError {
    fn from(e: hex::FromHexError) -> AppError {
        AppError::HexError(e)
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> AppError {
        AppError::IOError(e)
    }
}

impl From<eos_primitives::ParseNameError> for AppError {
    fn from(e: eos_primitives::ParseNameError) -> AppError {
        AppError::EosPrimitivesNamesError(e)
    }
}
