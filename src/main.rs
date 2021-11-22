// #![feature(plugin)]
// #![plugin(maud_macros)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate actix_rt;
extern crate env_logger;
extern crate lettre;
extern crate maud;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate toml;
extern crate url;
extern crate uuid;

use std::io::Read;

use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;

mod common;
mod db;
mod logging;
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
                debug!(
                    "Current directory: {}.",
                    std::env::current_dir()
                        .map(|d| d.to_str().unwrap_or("<unknown>").to_string())
                        .unwrap_or("<unknown>".to_string())
                );
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
            error!("Decode error while parsing config: {}.", decode_error);
            std::process::exit(1);
        }
        Ok(conf) => conf.config,
    }
}

async fn handle_favicon() -> HttpResponse {
    let response_body = include_bytes!("../bank.png");
    HttpResponse::Ok()
        .content_type("image/png")
        .body(actix_web::dev::Body::from_slice(response_body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logging::env_logger_init();
    let conf = load_config_or_exit();
    debug!("Config loaded:\n{:?}", conf);
    let addr = format!("{}:{}", "localhost", conf.port.unwrap());
    let pool = common::create_database_pool(
        &conf.db_host,
        conf.db_port,
        &conf.db_name,
        &conf.db_username,
        &conf.db_password,
    );
    HttpServer::new(move || {
        use actix_web::web::get;
        use actix_web::web::post;
        let app = App::new()
            .wrap(actix_web::middleware::Logger::default())
            .data(pool.clone())
            .data(conf.clone())
            .route("/", get().to(page::main::handle_main))
            .route("/favicon.ico", get().to(handle_favicon))
            .route("/about", get().to(page::about::handle_about))
            .route("/profile", get().to(page::profile::handle_profile))
            .route(
                "/accounts",
                get().to(page::bank_accounts::handle_bank_accounts),
            )
            .route("/add", get().to(page::add::handle_add))
            .route("/add", post().to(page::add::handle_post_add))
            .route(
                "/add-bank-account",
                get().to(page::add_bank_account::handle_get_add_bank_account),
            )
            .route(
                "/add-bank-account",
                post().to(page::add_bank_account::handle_post_add_bank_account),
            )
            .route("/currency", get().to(page::currency::handle_currency))
            .route("/delete", get().to(page::delete::handle_delete))
            .route("/edit", get().to(page::edit::handle_edit))
            .route("/edit", post().to(page::edit::handle_post_edit))
            .route(
                "/new-session",
                get().to(page::new_session::handle_new_session),
            )
            .route(
                "/new-session",
                post().to(page::new_session::handle_post_new_session),
            )
            .route(
                "/new-session/{token}",
                get().to(page::new_session::handle_get_new_session_with_token),
            )
            .route(
                "/logout",
                actix_web::web::get().to(page::logout::handle_get_logout),
            )
            .route(
                "/logout",
                actix_web::web::post().to(page::logout::handle_post_logout),
            )
            .route("/graph", get().to(page::graph::handle_graph))
            .route(
                "/export",
                actix_web::web::get().to(page::export::handle_export),
            )
            .route(
                "/export/{filename}",
                actix_web::web::get().to(page::export::handle_export_file),
            );
        app
    })
    .bind(addr)
    .expect("Failed to bind")
    .run()
    .await
}
