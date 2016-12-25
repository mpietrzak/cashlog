
#![feature(plugin)]
#![plugin(maud_macros)]

#[macro_use] extern crate iron;
#[macro_use] extern crate log;
// extern crate mount;
// extern crate staticfile;
extern crate env_logger;
extern crate hyper;
extern crate lettre;
extern crate logger;
extern crate maud;
extern crate params;
extern crate postgres;
extern crate psutil;
extern crate router;
extern crate time;
extern crate url;
extern crate uuid;
extern crate rustc_serialize;
extern crate toml;

// use std::path;
use iron::prelude::*;
use rustc_serialize::Decodable;
use std::io::Read;

mod common;
mod db;
mod model;
mod page;
mod tmpl;
mod util;

/// Load config or exit.
fn load_config_or_exit() -> model::Config {
    let config_filename = "cashlog.toml";
    let mut toml_source = String::new();
    {
        let mut f = match std::fs::File::open(config_filename) {
            Ok(f) => f,
            Err(e) => {
                error!("Failed to open config file {}: {}.", config_filename, e);
                debug!("Current directory: {}.", std::env::current_dir()
                    .map(|d| d.to_str().unwrap_or("<unknown>").to_string())
                    .unwrap_or("<unknown>".to_string()));
                std::process::exit(1);
            }
        };
        if let Err(e) = f.read_to_string(&mut toml_source) {
            error!("Failed to read config file: {}.", e);
            std::process::exit(1);
        }
    }
    let value: toml::Value = toml::Parser::new(&toml_source).parse().unwrap().get("config").unwrap().clone();
    let mut decoder = toml::Decoder::new(value);
    let conf_result = model::Config::decode(&mut decoder);
    match conf_result {
        Err(decode_error) => {
            debug!("Decode error: {}.", decode_error);
            std::process::exit(1);
        }
        Ok(conf) => conf
    }
}

struct ConfExtensionMiddleware {
    conf: model::Config
}

impl ConfExtensionMiddleware {
    fn new(conf: model::Config) -> ConfExtensionMiddleware {
        ConfExtensionMiddleware{conf: conf}
    }
}

impl iron::BeforeMiddleware for ConfExtensionMiddleware {
    fn before(&self, request: &mut iron::Request) -> iron::IronResult<()> {
        request.extensions.insert::<model::Config>(self.conf.clone());
        Ok(())
    }
}

fn main() {
    env_logger::init().unwrap();
    let conf = load_config_or_exit();
    let port = conf.port;
    debug!("Config loaded:\n{:?}", conf);
    let mut router = router::Router::new();
    router.get("/", page::main::handle_main, "main");
    router.get("/add", page::add::handle_add, "add");
    router.get("/logout", page::logout::handle_get_logout, "logout");
    router.get("/new-session", page::new_session::handle_new_session, "new-session");
    router.get("/new-session/:token", page::new_session::handle_get_new_session_token, "new-session-token");
    router.get("/profile", page::profile::handle_profile, "profile");
    router.post("/add", page::add::handle_post_add, "add");
    router.post("/new-session", page::new_session::handle_post_new_session, "new-session");
    // let mut mount = mount::Mount::new();
    // mount.mount("/static", staticfile::Static::new(path::Path::new("static/")));
    // mount.mount("/", router);
    let mut chain = Chain::new(router);
    let (logger_before, logger_after) = logger::Logger::new(None);
    let conf_extension_middleware = ConfExtensionMiddleware::new(conf);
    chain.link_before(logger_before);
    chain.link_before(conf_extension_middleware);
    chain.link_after(logger_after);
    let listen_addr = format!("localhost:{}", port.unwrap_or(14080));
    if let Err(e) = Iron::new(chain).http(&*listen_addr) {
        error!("Failed to start server: {}.", e)
    }
}
