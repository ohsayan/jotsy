/*
 * Copyright (c) 2022, Sayan Nandan <nandansayan@outlook.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/

use axum::{
    http::StatusCode,
    response::Html,
    routing::{get, post},
    AddExtensionLayer, Router,
};
use skytable::pool;
use std::{env, net::SocketAddr};
use tower_cookies::CookieManagerLayer;
// modules
mod config;
mod error;
mod handlers;
mod templates;
mod util;

const TABLE_AUTH: &str = "default:jotsyauth";
const TABLE_NOTES: &str = "default:jotsynotes";

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;
type JotsyResponseResult<T> = Result<T, error::ResponseError>;
type JotsyResponse = JotsyResponseResult<(StatusCode, Html<String>)>;

#[tokio::main]
async fn main() -> DynResult<()> {
    let cfg = config::Config::init()?;
    util::set_prod_mode(cfg.is_prod);
    // configure our logger
    env_logger::Builder::new()
        .parse_filters(&env::var("JOTSY_LOG").unwrap_or_else(|_| "info".to_owned()))
        .init();
    log::trace!(
        "Establishing connection to Skytable on: {}:{}",
        cfg.sky_host,
        cfg.sky_port
    );
    // get our skytable instance
    let pool = pool::get_async(cfg.sky_host, cfg.sky_port, 10).await?;
    // just attempt to get a connection
    pool.get().await?;
    log::trace!("Connected to Skytable pool");
    util::create_tables(&pool).await?;
    log::trace!("Created/reinitialized tables");
    // create the routes
    let mut router = Router::new()
        // this is our GET for /
        .route("/", get(handlers::root))
        .route("/create/note", post(handlers::app::create_note))
        .route("/login", post(handlers::login))
        .route("/login", get(handlers::login_get))
        .route("/logout", post(handlers::logout))
        .route("/account", get(handlers::account::account))
        .route("/delete/account", get(handlers::account::del_account_get))
        .route("/delete/account", post(handlers::account::del_account_post))
        .route("/delete/notes", get(handlers::account::del_notes_get))
        .route("/delete/notes", post(handlers::account::del_notes_post))
        // manually mount static assets
        .route(
            "/static/css/login.css",
            get(handlers::assets::index_login_css),
        )
        .route("/static/css/app.css", get(handlers::assets::index_app_css))
        .route("/static/js/login.js", get(handlers::assets::index_login_js))
        .route("/static/js/app.js", get(handlers::assets::index_app_js))
        .route("/favicon.ico", get(handlers::assets::favicon));
    if cfg.signup_enabled {
        router = router
            .route("/signup", post(handlers::signup))
            .route("/signup", get(handlers::signup_get));
    } else {
        router = router.route("/signup", get(handlers::signup::no_signup))
    }
    router = router
        // add a cookie "layer" (axum's way of customizing routing)
        .layer(CookieManagerLayer::new())
        // add the database "layer"
        .layer(AddExtensionLayer::new(pool));
    // now run the service
    let addr = SocketAddr::new(cfg.host.parse()?, cfg.port);
    log::info!("Running server on http://127.0.0.1:2022/");
    tokio::select! {
        _ = axum::Server::bind(&addr).serve(router.into_make_service()) => {}
        _ = tokio::signal::ctrl_c() => {}
    }
    log::info!("Finished serving. Goodbye!");
    Ok(())
}
