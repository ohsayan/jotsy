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

use axum::{extract::Extension, http::StatusCode};
use tower_cookies::{Cookie, Cookies};

use super::{COOKIE_TOKEN, COOKIE_USERNAME};
use crate::{
    templates::{LOGIN_PAGE, REDIRECT_HOME},
    util::{self, resp},
};

use skytable::{
    actions::AsyncActions,
    aio::Connection,
    ddl::AsyncDdl,
    error::{Error, SkyhashError},
    pool::AsyncPool,
    RespCode,
};

pub async fn root(cookies: Cookies, Extension(db): Extension<AsyncPool>) -> crate::RespTuple {
    // our database has hash(tokens) -> username
    // so we need to send the hash of the token and see if the returne value
    let mut con = match db.get().await {
        Ok(c) => c,
        Err(_) => return resp(StatusCode::INTERNAL_SERVER_ERROR, REDIRECT_HOME),
    };
    con.switch("default:jotsyauth").await.unwrap();
    let username = cookies.get(COOKIE_USERNAME);
    let token = cookies.get(COOKIE_TOKEN);
    match (username, token) {
        (Some(uname), Some(token)) => {
            let (uname_v, token_v) = (uname.value().to_owned(), token.value().to_owned());
            let verify_status = verify_user(&mut con, &uname_v, &token_v).await;
            drop(con); // return con to the pool; also helps borrowck
            match verify_status {
                VerifyStatus::No => {
                    // auth failed, so we should remove these cookies; else we'll keep on
                    // bumping into these
                    cookies.remove(Cookie::new(COOKIE_USERNAME, uname_v));
                    cookies.remove(Cookie::new(COOKIE_TOKEN, token_v));
                    resp(StatusCode::UNAUTHORIZED, REDIRECT_HOME)
                }
                VerifyStatus::Yes => super::app::app(uname_v, db).await,
                VerifyStatus::ServerError => resp(StatusCode::INTERNAL_SERVER_ERROR, REDIRECT_HOME),
            }
        }
        _ => resp(StatusCode::OK, LOGIN_PAGE),
    }
}

pub enum VerifyStatus {
    Yes,
    No,
    ServerError,
}

async fn verify_user<'a>(con: &mut Connection, uname: &'a str, token: &'a str) -> VerifyStatus {
    let hash = util::sha2(token);
    let ret: Result<String, Error> = con.get(hash).await;
    match ret {
        Ok(v) if v.eq(uname) => VerifyStatus::Yes,
        Ok(_) => VerifyStatus::No,
        Err(Error::SkyError(SkyhashError::Code(RespCode::NotFound))) => VerifyStatus::No,
        _ => VerifyStatus::ServerError,
    }
}
