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

use super::{COOKIE_TOKEN, COOKIE_USERNAME};
use crate::{
    templates::{LoginPage, NoticePage},
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
/// The login form
pub struct Login {
    username: String,
    password: String,
}

/// `GET` for `/login`
/// If any cookies are set, it will reload `/` to trigger authentication, else it will
/// return the login page
pub async fn login_get(cookies: Cookies) -> Html<String> {
    super::redirect_home_if_cookie_set(cookies, LoginPage::render_new(false)).await
}

/// Authenticate an user. **You must ensure that the user is verified before authenticating
/// them!**
/// This will:
/// - Generate a session token
/// - Store the hash of the session token in the auth table
/// - Set cookies `username` and `token` with a validity of 15 days
/// - Redirect the user to root `/`
pub(super) async fn authenticate(
    uname: String,
    cookies: &mut Cookies,
    con: &mut Connection,
) -> crate::JotsyResponse {
    // sweet, we're verified
    // generate a token
    let token = generate_token();
    // hash the token
    let token_hash = util::sha2(&token);
    // store the hash in the DB
    con.set(token_hash, &uname).await?;
    // now set cookies
    cookies.add(create_cookie(COOKIE_USERNAME, &uname));
    cookies.add(create_cookie(COOKIE_TOKEN, token));
    resp(
        StatusCode::OK,
        NoticePage::new_redirect("Logged in successfully."),
    )
}

/// `POST` for `/login`
/// This will:
/// - Attempt to verify the provided credentials
/// - If they are valid, it will call `authenticate`
/// - If not, it will return the login page with an error
pub async fn login(
    mut cookies: Cookies,
    Extension(db): Extension<AsyncPool>,
    Form(lgn): Form<Login>,
) -> crate::JotsyResponse {
    /*
    Login flow:
    1. Get the hashed password from the DB
    2. bcrypt::verify(hash_from_db, pass_from_form)
    3. If verified, generate a token
        a. Store hash(token) into DB
        b. Send token to browser
    4. If not verified, return to `/`
    */
    let mut con = db.get().await?;
    con.switch(crate::TABLE_AUTH).await?;
    let hash_from_db: Result<String, Error> = con.get(&lgn.username).await;
    match hash_from_db {
        Ok(v) if util::bcrypt_verify(&lgn.password, &v) => {
            authenticate(lgn.username, &mut cookies, &mut con).await
        }
        Ok(_) => {
            // nope, unverified
            resp(StatusCode::UNAUTHORIZED, LoginPage::render_new(true))
        }
        Err(Error::SkyError(SkyhashError::Code(RespCode::NotFound))) => {
            resp(StatusCode::NOT_FOUND, LoginPage::render_new(true))
        }
        Err(e) => {
            log::error!("Failed to log user in: {}", e);
            NoticePage::re500()
        }
    }
}

const CHARSET: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()-_=+[]{}\\|;:'\"<>./~";
const TOKEN_LEN: usize = 32;

/// Returns an authentication token
fn generate_token() -> String {
    (0..TOKEN_LEN)
        .map(|_| {
            let idx = rand::thread_rng().gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
