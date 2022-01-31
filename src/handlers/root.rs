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

use axum::response::{Html, IntoResponse};
use tower_cookies::Cookies;

const COOKIE_USERNAME: &str = "jotsy_user";
const COOKIE_TOKEN: &str = "jotsy_token";
const LOGIN_PAGE: &str = include_str!("../../templates/login.html");

pub async fn root(cookies: Cookies) -> impl IntoResponse {
    let username = cookies.get(COOKIE_USERNAME);
    let token = cookies.get(COOKIE_TOKEN);
    match (username, token) {
        (Some(uname), Some(token)) => super::app::app(uname, token).await,
        _ => login_page().await,
    }
}

async fn login_page() -> Html<String> {
    Html::from(LOGIN_PAGE.to_owned())
}
