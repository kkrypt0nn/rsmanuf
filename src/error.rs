use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    InvalidMacAddress,
    
    #[cfg(feature = "online")]
    #[cfg_attr(docsrs, doc(cfg(feature = "online")))]
    SendManufRequest(reqwest::Error),

    #[cfg(feature = "online")]
    #[cfg_attr(docsrs, doc(cfg(feature = "online")))]
    ManufResponseRetrieval(reqwest::Error),

    #[cfg(feature = "online")]
    #[cfg_attr(docsrs, doc(cfg(feature = "online")))]
    ManufResponseStatusCode(reqwest::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMacAddress => write!(f, "Invalid MAC address"),

            #[cfg(feature = "online")]
            Self::SendManufRequest(error) => write!(f, "Unable to fetch manufacturer: {error}"),

            #[cfg(feature = "online")]
            Self::ManufResponseRetrieval(error) => write!(f, "Unable to retrieve manufacturer response: {error}"),

            #[cfg(feature = "online")]
            Self::ManufResponseStatusCode(error) => write!(f, "Got an invalid status code while fetching manufacturer: {error}"),
        }
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            Self::InvalidMacAddress => None,

            #[cfg(feature = "online")]
            Self::SendManufRequest(error) | Self::ManufResponseRetrieval(error) | Self::ManufResponseStatusCode(error) => Some(error),
        }
    }
}