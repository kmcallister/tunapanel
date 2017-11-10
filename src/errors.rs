//! Custom error types.

use std::{io, net};
use hyper;
use serde_json;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        IO(io::Error);
        Hyper(hyper::error::Error);
        AddrParse(net::AddrParseError);
        SerdeJson(serde_json::Error);
    }
}
