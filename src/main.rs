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
use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tower_cookies::CookieManagerLayer;
// modules
mod handlers;
mod templates;
mod util;

const JOTSY_BIND_HOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const JOTSY_BIND_PORT: u16 = 2022;
const JOTSY_SKY_HOST: &str = "127.0.0.1";
const JOTSY_SKY_PORT: u16 = 2003;
const TABLE_AUTH: &str = "default:jotsyauth";
const TABLE_NOTES: &str = "default:jotsynotes";
const ENV_SKY_HOST: &str = "JOTSY_SKY_HOST";
const ENV_SKY_PORT: &str = "JOTSY_SKY_PORT";

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;
type RespTuple = (StatusCode, Html<String>);

#[tokio::main]
async fn main() -> DynResult<()> {
    let sky_host = env::var(ENV_SKY_HOST).unwrap_or(JOTSY_SKY_HOST.to_owned());
    let sky_port = env::var(ENV_SKY_PORT)
        .map(|p| p.parse())
        .unwrap_or(Ok(JOTSY_SKY_PORT))?;
    // configure our logger
    env_logger::Builder::new()
        .parse_filters(&env::var("JOTSY_LOG").unwrap_or_else(|_| "info".to_owned()))
        .init();
    log::trace!(
        "Establishing connection to Skytable on: {}:{}",
        sky_host,
        sky_port
    );
    // get our skytable instance
    let pool = pool::get_async(sky_host, sky_port, 10).await?;
    log::trace!("Connected to Skytable pool");
    util::create_tables(&pool).await?;
    log::trace!("Created/reinitialized tables");
    // this is our host:port
    let addr = SocketAddr::new(JOTSY_BIND_HOST, JOTSY_BIND_PORT);
    // create the routes
    let router = Router::new()
        // this is our GET for /
        .route("/", get(handlers::root))
        .route("/createnote", post(handlers::app::create_note))
        .route("/login", post(handlers::login))
        .route("/login", get(handlers::login_get))
        .route(
            "/static/css/login.css",
            get(handlers::assets::index_login_css),
        )
        .route("/static/js/login.js", get(handlers::assets::index_login_js))
        .route("/static/js/app.js", get(handlers::assets::index_app_js))
        .route("/signup", post(handlers::signup))
        .route("/signup", get(handlers::signup_get))
        .route("/logout", post(handlers::logout))
        // add a cookie "layer" (axum's way of customizing routing)
        .layer(CookieManagerLayer::new())
        // add the database "layer"
        .layer(AddExtensionLayer::new(pool));
    // now run the service
    log::info!("Running server on http://127.0.0.1:2022/");
    tokio::select! {
        _ = axum::Server::bind(&addr).serve(router.into_make_service()) => {}
        _ = tokio::signal::ctrl_c() => {}
    }
    log::info!("Finished serving. Goodbye!");
    Ok(())
}
