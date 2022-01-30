/*
 * Copyright 2022 Sayan Nandan <nandansayan@outlook.com>
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
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use std::{
    io::Error as IoError,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::services::ServeDir;

const LOGIN_PAGE: &str = include_str!("../templates/login.html");
const COOKIE_USERNAME: &str = "jotsy_user";
const COOKIE_TOKEN: &str = "jotsy_token";
const JOTSY_BIND_HOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const JOTSY_BIND_PORT: u16 = 2022;

#[tokio::main]
async fn main() {
    // this is our host:port
    let addr = SocketAddr::new(JOTSY_BIND_HOST, JOTSY_BIND_PORT);
    // create the routes
    let router = Router::new()
        // this is our GET for /
        .route("/", get(root))
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
        .layer(CookieManagerLayer::new());
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn root(cookies: Cookies) -> impl IntoResponse {
    let username = cookies.get(COOKIE_USERNAME);
    let token = cookies.get(COOKIE_TOKEN);
    match (username, token) {
        (Some(uname), Some(token)) => {
            println!("Logged in. {uname} and {token}");
            Html::from("<html>This is under construction</html>")
        }
        _ => {
            println!("Not logged in");
            Html::from(LOGIN_PAGE)
        }
    }
}
