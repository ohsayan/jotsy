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

use super::{COOKIE_TOKEN, COOKIE_USERNAME};
use crate::{
    templates::{LoginPage, RedirectHome},
    util::{self, create_cookie, resp},
};
use axum::{
    extract::{Extension, Form},
    http::StatusCode,
    response::Html,
};
use rand::Rng;
use serde::Deserialize;
use skytable::{
    actions::AsyncActions,
    aio::Connection,
    ddl::AsyncDdl,
    error::{Error, SkyhashError},
    pool::AsyncPool,
    RespCode,
};
use tower_cookies::Cookies;

#[derive(Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

pub async fn login_get(cookies: Cookies) -> Html<String> {
    super::redirect_home_if_cookie_set(cookies, LoginPage::new(false)).await
}

pub(super) async fn authenticate(
    uname: String,
    cookies: &mut Cookies,
    con: &mut Connection,
) -> crate::RespTuple {
    // sweet, we're verified
    // generate a token
    let token = generate_token();
    // hash the token
    let token_hash = util::sha2(&token);
    // store the hash in the DB
    con.set(token_hash, &uname).await.unwrap();
    // now set cookies
    cookies.add(create_cookie(COOKIE_USERNAME, &uname));
    cookies.add(create_cookie(COOKIE_TOKEN, token));
    resp(StatusCode::OK, RedirectHome::new("Logged in successfully."))
}

pub async fn login(
    mut cookies: Cookies,
    Extension(db): Extension<AsyncPool>,
    Form(lgn): Form<Login>,
) -> crate::RespTuple {
    /*
    Login flow:
    1. Get the hashed password from the DB
    2. bcrypt::verify(hash_from_db, pass_from_form)
    3. If verified, generate a token
        a. Store hash(token) into DB
        b. Send token to browser
    4. If not verified, return to `/`
    */
    let mut con = match db.get().await {
        Ok(c) => c,
        Err(e) => {
            log::error!("Failed to get connection from pool: {e}");
            return RedirectHome::re500();
        }
    };
    con.switch(crate::TABLE_AUTH).await.unwrap();
    let hash_from_db: Result<String, Error> = con.get(&lgn.username).await;
    match hash_from_db {
        Ok(v) if util::bcrypt_verify(&lgn.password, &v) => {
            authenticate(lgn.username, &mut cookies, &mut con).await
        }
        Ok(_) => {
            // nope, unverified
            resp(StatusCode::UNAUTHORIZED, LoginPage::new(true))
        }
        Err(Error::SkyError(SkyhashError::Code(RespCode::NotFound))) => {
            resp(StatusCode::NOT_FOUND, LoginPage::new(true))
        }
        Err(e) => {
            log::error!("Failed to log user in: {}", e);
            RedirectHome::re500()
        }
    }
}

const CHARSET: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()-_=+[]{}\\|;:'\"<>./~";
const TOKEN_LEN: usize = 32;

fn generate_token() -> String {
    (0..TOKEN_LEN)
        .map(|_| {
            let idx = rand::thread_rng().gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
