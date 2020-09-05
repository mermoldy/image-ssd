use crate::ms_coco;
use std::error;
use std::fmt;
use std::io;

pub struct GenericError {
    message: String,
}

impl std::error::Error for GenericError {}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GenericError {{ message: {}}}", self.message)
    }
}

pub enum Error {
    TensprFlowError(tensorflow::Status),
    ProtobufParseError(protobuf::text_format::ParseError),
    ShapeError(ndarray::ShapeError),
    DrawingError(piet::Error),
    LabelNotFound(ms_coco::LabelNotFound),
    IoError(io::Error),
    HttpError(minreq::Error),
    GenericError(GenericError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref e) => e.fmt(f),
            Error::HttpError(ref e) => e.fmt(f),
            Error::TensprFlowError(ref e) => e.fmt(f),
            Error::ProtobufParseError(ref e) => e.fmt(f),
            Error::ShapeError(ref e) => e.fmt(f),
            Error::DrawingError(ref e) => e.fmt(f),
            Error::LabelNotFound(ref e) => e.fmt(f),
            Error::GenericError(ref e) => e.fmt(f),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref e) => e.fmt(f),
            Error::HttpError(ref e) => e.fmt(f),
            Error::TensprFlowError(ref e) => e.fmt(f),
            Error::ProtobufParseError(ref e) => e.fmt(f),
            Error::ShapeError(ref e) => e.fmt(f),
            Error::DrawingError(ref e) => e.fmt(f),
            Error::LabelNotFound(ref e) => e.fmt(f),
            Error::GenericError(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::IoError(ref e) => Some(e),
            Error::HttpError(ref e) => Some(e),
            Error::TensprFlowError(ref e) => Some(e),
            Error::ProtobufParseError(ref e) => Some(e),
            Error::ShapeError(ref e) => Some(e),
            Error::DrawingError(ref e) => Some(e),
            Error::LabelNotFound(ref e) => Some(e),
            Error::GenericError(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<minreq::Error> for Error {
    fn from(err: minreq::Error) -> Error {
        Error::HttpError(err)
    }
}

impl From<tensorflow::Status> for Error {
    fn from(err: tensorflow::Status) -> Error {
        Error::TensprFlowError(err)
    }
}

impl From<protobuf::text_format::ParseError> for Error {
    fn from(err: protobuf::text_format::ParseError) -> Error {
        Error::ProtobufParseError(err)
    }
}

impl From<ndarray::ShapeError> for Error {
    fn from(err: ndarray::ShapeError) -> Error {
        Error::ShapeError(err)
    }
}

impl From<piet::Error> for Error {
    fn from(err: piet::Error) -> Error {
        Error::DrawingError(err)
    }
}

impl From<ms_coco::LabelNotFound> for Error {
    fn from(err: ms_coco::LabelNotFound) -> Error {
        Error::LabelNotFound(err)
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Error {
        Error::GenericError(GenericError {
            message: err.to_string(),
        })
    }
}
