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
    templates::{LOGIN_PAGE, REDIRECT_HOME},
    util::{create_cookie, resp},
};
use axum::{
    extract::{Extension, Form},
    http::StatusCode,
    response::Html,
};
use bcrypt::DEFAULT_COST;
use rand::Rng;
use serde::Deserialize;
use skytable::{actions::AsyncActions, ddl::AsyncDdl, error::Error, pool::AsyncPool};
use tower_cookies::Cookies;

#[derive(Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

pub async fn login(
    cookies: Cookies,
    Extension(db): Extension<AsyncPool>,
    Form(lgn): Form<Login>,
) -> (StatusCode, Html<String>) {
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
        Err(_) => return resp(StatusCode::INTERNAL_SERVER_ERROR, REDIRECT_HOME),
    };
    con.switch("default:jotsyauth").await.unwrap();
    let hash_from_db: Result<String, Error> = con.get(&lgn.username).await;
    match hash_from_db {
        Ok(v) if bcrypt::verify(&lgn.password, &v).unwrap() => {
            // sweet, we're verified
            // generate a token
            let token = generate_token();
            // hash the token
            let token_hash = bcrypt::hash(&token, DEFAULT_COST).unwrap();
            // store the hash in the DB
            con.set(token_hash, &lgn.username).await.unwrap();
            // now set cookies
            cookies.add(create_cookie(COOKIE_USERNAME, &lgn.username));
            cookies.add(create_cookie(COOKIE_TOKEN, token));
            resp(
                StatusCode::OK,
                format!(
                    "<h1>Hey, {name}! We've authenticated you and saved your session",
                    name = lgn.username
                ),
            )
        }
        Ok(_) => {
            // nope, unverified
            resp(StatusCode::UNAUTHORIZED, LOGIN_PAGE)
        }
        Err(_) => resp(StatusCode::INTERNAL_SERVER_ERROR, REDIRECT_HOME),
    }
}

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~";
const TOKEN_LEN: usize = 24;

fn generate_token() -> String {
    (0..TOKEN_LEN)
        .map(|_| {
            let idx = rand::thread_rng().gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
