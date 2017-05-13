use std::default::Default;
use std::io::prelude::*;

use hyper::{Get, Post};
use hyper::status::StatusCode;
use hyper::server::{Server, Request, Response};
use hyper::uri::RequestUri::AbsolutePath;
use serde_json;

use errors::Result;
use panel::{Panel, panel_html};

/// Server configuration.
pub struct ServerConfig {
    /// Whether to print messages, such as the listen address.
    pub verbose: bool,

    /// IP address and port on which to listen.
    ///
    /// This is a string of the form `"127.0.0.1:1337"`.
    /// Use `"0.0.0.0:1337"` to listen on all network
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
    where P: Panel,
          F: Fn(P) + Sync + Send + 'static,
{
    serve_with_config::<P, _>(Default::default(), f)
}

/// Serve a panel on a specific port.
///
/// `f` is the callback for panel updates.
pub fn serve_on_port<P, F>(port: u16, f: F) -> Result<()>
    where P: Panel,
          F: Fn(P) + Sync + Send + 'static,
{
    let config = ServerConfig{
        verbose: true,
        listen_on: format!("127.0.0.1:{}", port)
    };

    serve_with_config::<P, _>(config, f)
}

/// Serve a panel and configure the server.
///
/// `f` is the callback for panel updates.
pub fn serve_with_config<P, F>(cfg: ServerConfig, f: F) -> Result<()>
    where P: Panel,
          F: Fn(P) + Sync + Send + 'static,
{
    let panel_html = panel_html::<P>();

    let server = Server::http(&cfg.listen_on)?;

    let handle_update = move |req: &mut Request| -> Result<()> {
        let mut json = String::new();
        req.read_to_string(&mut json)?;

        let panel = serde_json::from_str(&json)?;
        f(panel);

        Ok(())
    };

    let _guard = server.handle(move |mut req: Request, mut res: Response| {
        // Clone to avoid borrowck, idgaf
        match req.uri.clone() {
            AbsolutePath(ref path) => match (&req.method, &path[..]) {
                (&Get, "/") => {
                    if let Err(e) = res.send(panel_html.as_bytes()) {
                        error!("Could not send panel page: {}", e);
                    }
                },

                (&Post, "/update") => {
                    if let Err(e) = handle_update(&mut req) {
                        warn!("Ignoring erroneous update: {}", e);
                        *res.status_mut() = StatusCode::BadRequest;
                    }
                },

                _ => {
                    *res.status_mut() = StatusCode::NotFound;
                }
            },

            _ => {
                *res.status_mut() = StatusCode::NotFound;
            }
        }
    });

    if cfg.verbose {
        println!("Listening on {}", cfg.listen_on);
    }

    Ok(())
}
