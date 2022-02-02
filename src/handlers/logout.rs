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

use crate::{
    templates::RedirectHome,
    util::{self, resp},
};
use axum::{
    extract::{Extension, Form},
    http::StatusCode,
};
use serde::Deserialize;
use skytable::{actions::AsyncActions, pool::AsyncPool};
use tower_cookies::{Cookie, Cookies};

#[derive(Deserialize)]
pub struct Empty {}

pub async fn logout(
    Form(_): Form<Empty>,
    cookies: Cookies,
    Extension(db): Extension<AsyncPool>,
) -> crate::RespTuple {
    let mut con = match db.get().await {
        Ok(c) => c,
        Err(_) => {
            return resp(
                StatusCode::INTERNAL_SERVER_ERROR,
                RedirectHome::new("Internal server error."),
            )
        }
    };
    let c_user = cookies.get(super::COOKIE_USERNAME);
    let c_token = cookies.get(super::COOKIE_TOKEN);
    match (c_user, c_token) {
        (Some(user), Some(token)) => {
            let user = user.value().to_owned();
            let token = token.value().to_owned();
            // let's attempt to remove this
            let _ = con.del(util::sha2(&token)).await;
            // now remove these cookies
            cookies.remove(Cookie::new(super::COOKIE_USERNAME, user));
            cookies.remove(Cookie::new(super::COOKIE_TOKEN, token));
            resp(
                StatusCode::OK,
                RedirectHome::new("Logged out successfully."),
            )
        }
        (Some(cookie), None) | (None, Some(cookie)) => {
            let (c_key, c_v) = (cookie.name().to_owned(), cookie.value().to_owned());
            // random cookies, just pop them
            cookies.remove(Cookie::new(c_key, c_v));
            resp(
                StatusCode::OK,
                RedirectHome::new("Invalid cookies detected and removed."),
            )
        }
        (None, None) => resp(
            StatusCode::NOT_ACCEPTABLE,
            RedirectHome::new("Unexpected request to /logout"),
        ),
    }
}
