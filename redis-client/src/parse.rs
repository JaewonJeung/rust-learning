use crate::frame::Frame;
use bytes::Bytes;
use std::{fmt, str, vec};

/// Utility for parsing a command
///
/// Commands are represented as array frames. Each entry in the frame is a
/// "token". A `Parse` is initialized with the array frame and provides a
/// cursor-like API. Each command struct includes a `parse_frame` method that
/// uses a `Parse` to extract its fields.
#[derive(Debug)]
pub(crate) struct Parse {
    /// Array frame iterator.
    parts: vec::IntoIter<Frame>,
}

/// Error encountered while parsing a frame.
///
/// Only `EndOfStream` errors are handled at runtime. All other errors result in
/// the connection being terminated.
#[derive(Debug)]
pub(crate) enum ParseError {
    /// Attempting to extract a value failed due to the frame being fully
    /// consumed.
    EndOfStream,

    /// All other errors
    Other(crate::Error),
}

impl Parse {
    /// Create a new `Parse` to parse the contents of `frame`.
    ///
    /// Returns `Err` if `frame` is not an array frame.
    pub(crate) fn new(frame: Frame) -> Result<Parse, ParseError> {
        let array = match frame {
            Frame::Array(array) => {
                dlog!("Parse::new - array len={}", array.len());
                array
            }
            frame => {
                return Err(ParseError::Other(anyhow::anyhow!(format!(
                    "protocol error; expected array, got {:?}",
                    frame
                ))));
            }
        };

        Ok(Parse {
            parts: array.into_iter(),
        })
    }

    /// Return the next entry. Array frames are arrays of frames, so the next
    /// entry is a frame.
    fn next(&mut self) -> Result<Frame, ParseError> {
        let next = self.parts.next();
        dlog!("Parse::next - has_next={} ", next.is_some());
        next.ok_or(ParseError::EndOfStream)
    }

    /// Return the next entry as a string.
    ///
    /// If the next entry cannot be represented as a String, then an error is returned.
    pub(crate) fn next_string(&mut self) -> Result<String, ParseError> {
        match self.next()? {
            // Both `Simple` and `Bulk` representation may be strings. Strings
            // are parsed to UTF-8.
            //
            // While errors are stored as strings, they are considered separate
            // types.
            Frame::Simple(s) => {
                dlog!("Parse::next_string - simple='{}'", s);
                Ok(s)
            }
            Frame::Bulk(data) => {
                let s = str::from_utf8(&data[..])
                    .map(|s| s.to_string())
                    .map_err(|_| {
                        ParseError::Other(anyhow::anyhow!("protocol error; invalid string"))
                    })?;
                dlog!("Parse::next_string - bulk='{}'", s);
                Ok(s)
            }
            frame => Err(ParseError::Other(anyhow::anyhow!(format!(
                "protocol error; expected simple frame or bulk frame, got {:?}",
                frame
            )))),
        }
    }

    /// Return the next entry as raw bytes.
    ///
    /// If the next entry cannot be represented as raw bytes, an error is
    /// returned.
    pub(crate) fn next_bytes(&mut self) -> Result<Bytes, ParseError> {
        match self.next()? {
            Frame::Simple(s) => {
                let b = Bytes::from(s.clone().into_bytes());
                dlog!("Parse::next_bytes - simple len={} ", b.len());
                Ok(b)
            }
            Frame::Bulk(data) => {
                dlog!("Parse::next_bytes - bulk len={} ", data.len());
                Ok(data)
            }
            frame => Err(ParseError::Other(anyhow::anyhow!(format!(
                "protocol error; expected simple frame or bulk frame, got {:?}",
                frame
            )))),
        }
    }

    /// Return the next entry as an integer.
    ///
    /// This includes `Simple`, `Bulk`, and `Integer` frame types. `Simple` and
    /// `Bulk` frame types are parsed.
    ///
    /// If the next entry cannot be represented as an integer, then an error is
    /// returned.
    pub(crate) fn next_int(&mut self) -> Result<u64, ParseError> {
        use atoi::atoi;

        const MSG: &str = "protocol error; invalid number";

        match self.next()? {
            // An integer frame type is already stored as an integer.
            Frame::Integer(v) => {
                dlog!("Parse::next_int - integer={}", v);
                Ok(v)
            }
            // Simple and bulk frames must be parsed as integers. If the parsing
            // fails, an error is returned.
            Frame::Simple(data) => {
                let v = atoi::<u64>(data.as_bytes())
                    .ok_or_else(|| ParseError::Other(anyhow::anyhow!(MSG)))?;
                dlog!("Parse::next_int - simple parsed={}", v);
                Ok(v)
            }
            Frame::Bulk(data) => {
                let v =
                    atoi::<u64>(&data).ok_or_else(|| ParseError::Other(anyhow::anyhow!(MSG)))?;
                dlog!("Parse::next_int - bulk parsed={}", v);
                Ok(v)
            }
            frame => Err(ParseError::Other(anyhow::anyhow!(format!(
                "protocol error; expected int frame but got {:?}",
                frame
            )))),
        }
    }

    /// Ensure there are no more entries in the array
    pub(crate) fn finish(&mut self) -> Result<(), ParseError> {
        if self.parts.next().is_none() {
            dlog!("Parse::finish - end of frame");
            Ok(())
        } else {
            Err(ParseError::Other(anyhow::anyhow!(
                "protocol error; expected end of frame, but there was more"
            )))
        }
    }
}

impl From<String> for ParseError {
    fn from(src: String) -> ParseError {
        ParseError::Other(anyhow::anyhow!(src))
    }
}

impl From<&str> for ParseError {
    fn from(src: &str) -> ParseError {
        ParseError::Other(anyhow::anyhow!(src.to_string()))
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EndOfStream => "protocol error; unexpected end of stream".fmt(f),
            ParseError::Other(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for ParseError {}
