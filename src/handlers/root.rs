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

use axum::extract::Extension;
use tower_cookies::Cookies;

use super::{COOKIE_TOKEN, COOKIE_USERNAME};
use crate::{error::ResponseError, templates::LoginPage, util};

use skytable::{actions::AsyncActions, aio::Connection, ddl::AsyncDdl, pool::AsyncPool};

pub async fn root(
    mut cookies: Cookies,
    Extension(db): Extension<AsyncPool>,
) -> crate::JotsyResponse {
    // our database has hash(tokens) -> username
    // so we need to send the hash of the token and see if the returne value
    let mut con = db.get().await?;
    let uname = verify_user_or_error(&mut con, &mut cookies).await?;
    drop(con);
    super::app::app(uname, db).await
}

pub(super) async fn verify_user_or_error(
    con: &mut Connection,
    cookies: &mut Cookies,
) -> crate::JotsyResponseResult<String> {
    con.switch(crate::TABLE_AUTH).await?;
    let username = cookies.get(COOKIE_USERNAME);
    let token = cookies.get(COOKIE_TOKEN);
    match (username, token) {
        (Some(uname), Some(token)) => {
            let (uname_v, token_v) = (uname.value().to_owned(), token.value().to_owned());
            let verified = verify_user(con, &uname_v, &token_v).await?;
            drop(con); // return con to the pool; also helps borrowck
            if verified {
                Ok(uname.value().to_string())
            } else {
                // auth failed, so we should remove these cookies; else we'll keep on
                // bumping into these
                cookies.remove(util::create_cookie(COOKIE_USERNAME, uname_v));
                cookies.remove(util::create_cookie(COOKIE_TOKEN, token_v));
                Err(ResponseError::AppError(
                    "Found invalid or outdated cookies.",
                ))
            }
        }
        _ => Err(ResponseError::Redirect(LoginPage::new(false))),
    }
}

async fn verify_user<'a>(
    con: &mut Connection,
    uname: &'a str,
    token: &'a str,
) -> crate::JotsyResponseResult<bool> {
    let hash = util::sha2(token);
    con.get(hash).await.map(|ret: String| Ok(ret.eq(uname)))?
}
