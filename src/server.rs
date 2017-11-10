use std::default::Default;
use std::rc::Rc;
use std::marker::PhantomData;

use futures::{future, Future, Stream};

use bytes::Bytes;

use hyper::{self, Get, Post, StatusCode};
use hyper::server::{Http, Request, Response, Service};
use hyper::header::ContentLength;
use serde_json;

use errors::{Error, Result};
use panel::{panel_html, Panel};

/// Server configuration.
pub struct ServerConfig {
    /// Whether to print messages, such as the listen address.
    ///
    /// Default: `true`
    pub verbose: bool,

    /// IP address and port on which to listen.
    ///
    /// Default: `"127.0.0.1:1337"`
    ///
    /// Use e.g. `"0.0.0.0:1337"` to listen on all network
    /// interfaces.
    pub listen_on: String,
}

impl Default for ServerConfig {
    fn default() -> ServerConfig {
        ServerConfig {
            verbose: true,
            listen_on: "127.0.0.1:1337".to_owned(),
        }
    }
}

/// Serve a panel.
///
/// `f` is the callback for panel updates.
pub fn serve<P, F>(f: F) -> Result<()>
where
    P: Panel + 'static,
    F: Fn(P) + Sync + Send + 'static,
{
    serve_with_config::<P, _>(Default::default(), f)
}

/// Serve a panel and configure the server.
///
/// `f` is the callback for panel updates.
pub fn serve_with_config<P, F>(cfg: ServerConfig, f: F) -> Result<()>
where
    P: Panel + 'static,
    F: Fn(P) + Sync + Send + 'static,
{
    let handler = ConfiguredServer::<P, F>::new(f);

    let addr = cfg.listen_on.parse()?;

    let server = Http::new().bind(&addr, move || Ok(handler.clone()))?;

    if cfg.verbose {
        println!("Listening on {}", cfg.listen_on);
    }

    server.run()?;

    Ok(())
}

struct ConfiguredServer<P, F> {
    panel_html: Bytes,
    callback: Rc<F>,
    _phantom: PhantomData<P>,
}


impl<P, F> Clone for ConfiguredServer<P, F> {
    fn clone(&self) -> Self {
        return ConfiguredServer {
            panel_html: self.panel_html.clone(),
            callback: self.callback.clone(),
            _phantom: PhantomData,
        };
    }

    fn clone_from(&mut self, source: &Self) {
        self.panel_html.clone_from(&source.panel_html);
        self.callback.clone_from(&source.callback);
    }
}

impl<P, F> ConfiguredServer<P, F>
where
    P: Panel,
{
    pub fn new(callback: F) -> Self {
        return ConfiguredServer {
            panel_html: panel_html::<P>().into(),
            callback: Rc::new(callback),
            _phantom: PhantomData,
        };
    }
}

impl<P, F> Service for ConfiguredServer<P, F>
where
    P: Panel + 'static,
    F: Fn(P) + Sync + Send + 'static,
{
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        // to_owned to avoid borrowck, idgaf
        let path = req.uri().path().to_owned();
        match (req.method(), &*path) {
            (&Get, "/") => Box::new(future::ok(
                Response::new()
                    .with_header(ContentLength(self.panel_html.len() as u64))
                    .with_body(self.panel_html.clone()), // reference-counted pointer clone
            )),
            (&Post, "/update") => {
                let callback = self.callback.clone(); // reference-counted pointer clone
                Box::new(
                    req.body()
                        .concat2()
                        .map_err(Into::into)
                        .and_then(move |body| {
                            let panel = serde_json::from_slice(&body)?;
                            (callback)(panel);

                            Ok(Response::new().with_status(StatusCode::Ok))
                        })
                        .or_else(|error: Error| {
                            warn!("Ignoring erroneous update: {}", error);

                            Ok(Response::new().with_status(StatusCode::BadRequest))
                        }),
                )
            }
            _ => Box::new(future::ok(
                Response::new().with_status(StatusCode::NotFound),
            )),
        }
    }
}
