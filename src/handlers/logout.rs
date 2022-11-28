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

use crate::{
    templates::NoticePage,
    util::{self, resp, Empty},
};
use axum::{
    extract::{Extension, Form},
    http::StatusCode,
};
use skytable::{actions::AsyncActions, ddl::AsyncDdl, pool::AsyncPool};
use tower_cookies::Cookies;

use super::{COOKIE_USERNAME, COOKIE_TOKEN};

/// `POST` for `/logout`
pub async fn logout(
    Form(_): Form<Empty>,
    cookies: Cookies,
    Extension(db): Extension<AsyncPool>,
) -> crate::JotsyResponse {
    self::logout_core(cookies, "Logged out successfully", db).await
}

/// The main logic for a logout procedure. This will:
/// - Get the cookies
/// - Will attempt to remove hash(token) from the DB
///     - If this succeeds, it will remove the cookies
/// - If there are either of `username` or `token` cookies set, then remove them
/// - If no cookies are set, it will simply return a NOT_ACCEPTABLE error because
/// you aren't expected to `POST` to `/logout` without either
/// - Redirects to `/`
pub async fn logout_core(
    cookies: Cookies,
    redirect_message: &'static str,
    db: AsyncPool,
) -> crate::JotsyResponse {
    let mut con = db.get().await?;
    let c_user = cookies.get(super::COOKIE_USERNAME);
    let c_token = cookies.get(super::COOKIE_TOKEN);
    con.switch(crate::TABLE_AUTH).await?;
    match (c_user, c_token) {
        (Some(_), Some(token_c)) => {
            let token = token_c.value().to_owned();
            // let's attempt to remove this
            let del = con.del(util::sha2(token)).await?;
            // now remove these cookies
            if del == 1 {
                cookies.remove(util::null_cookie(COOKIE_USERNAME));
                cookies.remove(util::null_cookie(COOKIE_TOKEN));
            }
            resp(StatusCode::OK, NoticePage::new_redirect(redirect_message))
        }
        (Some(cookie), None) | (None, Some(cookie)) => {
            // random cookies, just pop them
            cookies.remove(util::null_cookie(cookie.name()));
            resp(
                StatusCode::OK,
                NoticePage::new_redirect("Invalid cookies detected and removed."),
            )
        }
        (None, None) => resp(
            StatusCode::NOT_ACCEPTABLE,
            NoticePage::new_redirect("Unexpected request to /logout"),
        ),
    }
}
