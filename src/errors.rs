use std::io;
use hyper;
use serde_json;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        IO(io::Error);
        Hyper(hyper::error::Error);
        SerdeJson(serde_json::Error);
    }
}
