/*
 * Copyright 2022, Sayan Nandan <nandansayan@outlook.com>
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
    routing::{get, get_service, post},
    AddExtensionLayer, Router,
};
use skytable::pool;
use std::{
    io::Error as IoError,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
// modules
mod handlers;
mod util;
mod templates;

const JOTSY_BIND_HOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const JOTSY_BIND_PORT: u16 = 2022;

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> DynResult<()> {
    // get our skytable instance
    let pool = pool::get_async("127.0.0.1", 2003, 10).await?;
    util::create_tables(&pool).await?;
    // this is our host:port
    let addr = SocketAddr::new(JOTSY_BIND_HOST, JOTSY_BIND_PORT);
    // create the routes
    let router = Router::new()
        // this is our GET for /
        .route("/", get(handlers::root))
        .route("/login", post(handlers::login))
        // mount our static assets
        .nest(
            "/static",
            get_service(ServeDir::new("static/")).handle_error(|error: IoError| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled fs error: {}", error),
                )
            }),
        )
        // add a cookie "layer" (axum's way of customizing routing)
        .layer(CookieManagerLayer::new())
        // add the database "layer"
        .layer(AddExtensionLayer::new(pool));
    // now run the service
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}
