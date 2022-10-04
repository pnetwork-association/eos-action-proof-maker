use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Custom(String),
    IOError(std::io::Error),
    HexError(hex::FromHexError),
    SerdeJsonError(serde_json::error::Error),
    EosChainWriteError(eos_chain::WriteError),
    EosPrimitivesError(eos_chain::error::Error),
    EosPrimitivesNamesError(eos_chain::ParseNameError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            AppError::Custom(ref msg) => msg.to_string(),
            AppError::HexError(ref e) => format!("Hex Error: {}", e),
            AppError::IOError(ref e) => format!("I/O Error: {}", e),
            AppError::SerdeJsonError(ref e) => format!("Serde JSON error: {}", e),
            AppError::EosPrimitivesError(ref e) => format!("Eos Primitives Error: {:?}", e),
            AppError::EosChainWriteError(ref e) => format!("Eos chain write error: {:?}", e),
            AppError::EosPrimitivesNamesError(ref e) => {
                format!("Eos Primitives Names Error: {:?}", e)
            }
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

impl From<eos_chain::ParseNameError> for AppError {
    fn from(e: eos_chain::ParseNameError) -> AppError {
        AppError::EosPrimitivesNamesError(e)
    }
}

impl From<eos_chain::error::Error> for AppError {
    fn from(e: eos_chain::error::Error) -> AppError {
        AppError::EosPrimitivesError(e)
    }
}

impl From<serde_json::error::Error> for AppError {
    fn from(e: serde_json::error::Error) -> AppError {
        AppError::SerdeJsonError(e)
    }
}

impl From<eos_chain::WriteError> for AppError {
    fn from(e: eos_chain::WriteError) -> AppError {
        AppError::EosChainWriteError(e)
    }
}
