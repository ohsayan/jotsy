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

const LOGIN_PAGE: &str = include_str!("../templates/login.html");

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use tower_http::services::ServeDir;

use std::{io::Error as IoError, net::SocketAddr};

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(root)).nest(
        "/static",
        get_service(ServeDir::new("static/")).handle_error(|error: IoError| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled fs error: {}", error),
            )
        }),
    );
    let addr = SocketAddr::from(([127, 0, 0, 1], 2022));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn root() -> impl IntoResponse {
    Html::from(LOGIN_PAGE)
}
