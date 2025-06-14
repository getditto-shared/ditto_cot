
use thiserror::Error;
use quick_xml;
use quick_xml::events::attributes::AttrError;

#[derive(Error, Debug)]
pub enum CotError {
    #[error("XML error: {0}")]
    XmlError(String),
    
    #[error("XML parse error: {0}")]
    XmlParse(String),
    
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    #[error("Invalid format: {0}")]
    Format(String),
    
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
}

impl From<quick_xml::Error> for CotError {
    fn from(err: quick_xml::Error) -> Self {
        CotError::XmlError(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for CotError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        CotError::XmlError(err.to_string())
    }
}

impl From<AttrError> for CotError {
    fn from(err: AttrError) -> Self {
        CotError::XmlError(err.to_string())
    }
}