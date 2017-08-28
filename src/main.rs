
// #![feature(plugin)]
// #![plugin(maud_macros)]

#![feature(proc_macro)]

#[macro_use]
extern crate iron;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
// extern crate mount;
// extern crate staticfile;
extern crate cookie;
extern crate env_logger;
extern crate hyper;
extern crate lettre;
extern crate logger;
extern crate maud;
extern crate params;
extern crate plugin;
extern crate postgres;
extern crate psutil;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate router;
extern crate time;
extern crate url;
extern crate uuid;
extern crate toml;
extern crate mime;

// use std::path;
use iron::prelude::*;
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
                debug!("Current directory: {}.",
                       std::env::current_dir()
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
    // This inline struct is only so that I can have top level "config" key
    // in toml config file, without having to go through Value object.
    #[derive(Debug, Deserialize)]
    struct ConfigWrapper {
        config: model::Config,
    }
    match toml::from_str::<ConfigWrapper>(&toml_source) {
        Err(decode_error) => {
            debug!("Decode error: {}.", decode_error);
            std::process::exit(1);
        }
        Ok(conf) => conf.config,
    }
}

struct ConfExtensionMiddleware {
    conf: model::Config,
}

impl ConfExtensionMiddleware {
    fn new(conf: model::Config) -> ConfExtensionMiddleware {
        ConfExtensionMiddleware { conf: conf }
    }
}

impl iron::BeforeMiddleware for ConfExtensionMiddleware {
    fn before(&self, request: &mut iron::Request) -> iron::IronResult<()> {
        request.extensions.insert::<model::Config>(self.conf.clone());
        Ok(())
    }
}

pub fn favicon(_: &mut iron::Request) -> iron::IronResult<iron::Response> {
    let content_type = "image/png".parse::<mime::Mime>().unwrap();
    let response_body = include_bytes!("../bank.png");
    let resp = iron::Response::with((iron::status::Ok, content_type, &response_body[..]));
    Ok(resp)
}

fn main() {
    env_logger::init().unwrap();
    let conf = load_config_or_exit();
    let port = conf.port;
    debug!("Config loaded:\n{:?}", conf);
    let mut router = router::Router::new();
    router.get("/favicon.ico", favicon, "favicon");
    router.get("/", page::main::handle_main, "main");
    router.get("/about", page::about::handle_about, "about");
    router.get("/accounts",
               page::bank_accounts::handle_bank_accounts,
               "bank-accounts");
    router.get("/add", page::add::handle_add, "add");
    router.get("/add-bank-account",
               page::add_bank_account::handle_get_add_bank_account,
               "add-bank-account");
    router.get("/currency", page::currency::handle_currency, "currency");
    router.get("/delete", page::delete::handle_delete, "delete");
    router.get("/edit", page::edit::handle_edit, "edit");
    router.get("/export", page::export::handle_export, "export");
    router.get("/export/:filename",
               page::export::handle_export_file,
               "export-file");
    router.get("/graph", page::graph::handle_graph, "graph");
    router.get("/logout", page::logout::handle_get_logout, "logout");
    router.get("/new-session",
               page::new_session::handle_new_session,
               "new-session");
    router.get("/new-session/:token",
               page::new_session::handle_get_new_session_token,
               "new-session-token");
    router.get("/profile", page::profile::handle_profile, "profile");
    router.post("/add", page::add::handle_post_add, "add");
    router.post("/add-bank-account",
               page::add_bank_account::handle_post_add_bank_account,
               "add-bank-account");
    router.post("/edit", page::edit::handle_post_edit, "edit");
    router.post("/logout", page::logout::handle_post_logout, "logout");
    router.post("/new-session",
                page::new_session::handle_post_new_session,
                "new-session");
    // let mut mount = mount::Mount::new();
    // mount.mount("/static", staticfile::Static::new(path::Path::new("static/")));
    // mount.mount("/", router);
    let mut chain = Chain::new(router);
    let (logger_before, logger_after) = logger::Logger::new(None);
    let conf_extension_middleware = ConfExtensionMiddleware::new(conf);
    chain.link_before(logger_before);
    chain.link_before(conf_extension_middleware);
    let pool = common::create_database_pool();
    let database_pool_middleware = common::DatabasePoolMiddleware { pool: pool };
    chain.link_before(database_pool_middleware);
    chain.link_after(logger_after);
    let listen_addr = format!("localhost:{}", port.unwrap_or(14080));
    if let Err(e) = Iron::new(chain).http(&*listen_addr) {
        error!("Failed to start server: {}.", e)
    }
}
