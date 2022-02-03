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
    templates::{RedirectHome, SignupPage},
    util::{self, resp},
};
use axum::{
    extract::{Extension, Form},
    http::StatusCode,
    response::Html,
};
use serde::Deserialize;
use skytable::{actions::AsyncActions, ddl::AsyncDdl, pool::AsyncPool, query, Element, RespCode};
use tower_cookies::Cookies;

#[derive(Deserialize)]
pub struct SignupForm {
    username: String,
    password: String,
    vpassword: String,
}

pub async fn signup_get(cookies: Cookies) -> Html<String> {
    super::redirect_home_if_cookie_set(cookies, SignupPage::new(false)).await
}

pub async fn signup(
    Form(data): Form<SignupForm>,
    mut cookies: Cookies,
    Extension(db): Extension<AsyncPool>,
) -> crate::RespTuple {
    /*
    Signup flow:
    1. Hash the password (TODO: report error if vpassword != password)
    2. Now `set` username->hashed passowrd
        a. If this fails, username is taken
        b. If this succeeds, username is available and we've created an user
    3. Now call super::login::authenticate(username, &mut cookies, &mut connection)
    */
    let hash = util::bcrypt_hash(&data.password);
    let mut con = match db.get().await {
        Ok(c) => c,
        Err(e) => {
            log::error!("Failed to get connection from pool: {e}");
            return resp(StatusCode::INTERNAL_SERVER_ERROR, RedirectHome::e500());
        }
    };
    con.switch(crate::TABLE_AUTH).await.unwrap();
    match con.set(data.username.clone(), hash).await {
        Ok(created_new) if created_new => {
            // cool, we did well
            log::info!("New user `{uname}` created.", uname = data.username);
            let ret =
                super::login::authenticate(data.username.clone(), &mut cookies, &mut con).await;
            con.switch(crate::TABLE_NOTES).await.unwrap();
            // attempt to create an empty list
            let query = query!("LSET", data.username);
            if let Ok(Element::RespCode(RespCode::Okay | RespCode::OverwriteError)) =
                con.run_simple_query(&query).await
            {
                ret
            } else {
                resp(StatusCode::INTERNAL_SERVER_ERROR, RedirectHome::e500())
            }
        }
        Ok(_) => {
            // nope, username is taken
            resp(StatusCode::CONFLICT, SignupPage::new(true))
        }
        Err(e) => {
            // server error
            log::error!("Failed to create user: {e}");
            resp(StatusCode::INTERNAL_SERVER_ERROR, RedirectHome::e500())
        }
    }
}
