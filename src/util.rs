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

use axum::{http::StatusCode, response::Html};
use cookie::SameSite;
use sha2::{Digest, Sha256};
use skytable::pool::AsyncPool;
use skytable::Query;
use time::{Duration, OffsetDateTime};
use tower_cookies::Cookie;

const CREATE_JOTSY_TABLE_AUTH: &str = "create table default:jotsyauth keymap(binstr,binstr)";
const CREATE_JOTSY_TABLE_NOTES: &str = "create table default:jotsynotes keymap(str,list<str>)";

pub fn query(q: &str) -> Query {
    let q: Vec<&str> = q.split_ascii_whitespace().collect();
    Query::from(q)
}

use skytable::{error::errorstring::ERR_ALREADY_EXISTS, Element, RespCode};

pub async fn create_tables(pool: &AsyncPool) -> crate::DynResult<()> {
    let mut con = pool.get().await?;
    let r1 = con
        .run_simple_query(&query(CREATE_JOTSY_TABLE_AUTH))
        .await?;
    let r2 = con
        .run_simple_query(&query(CREATE_JOTSY_TABLE_NOTES))
        .await?;
    let check_error = |e| match e {
        Element::RespCode(RespCode::Okay) => {}
        Element::RespCode(RespCode::ErrorString(s)) if s.eq(ERR_ALREADY_EXISTS) => {}
        _ => panic!("Unexpected response: {:?}", e),
    };
    check_error(r1);
    check_error(r2);
    Ok(())
}

pub fn resp(code: StatusCode, body: impl ToString) -> (StatusCode, Html<String>) {
    (code, Html::from(body.to_string()))
}

pub fn create_cookie(name: impl ToString, value: impl ToString) -> Cookie<'static> {
    let mut c = Cookie::new(name.to_string(), value.to_string());
    #[allow(deprecated)] // this is because of the tower-cookies crate
    let mut now = OffsetDateTime::now();
    now += Duration::days(15);
    c.set_expires(now);
    c.set_same_site(SameSite::Strict);
    c.set_secure(true);
    c.set_http_only(true);
    c
}

pub fn bcrypt_hash(input: impl AsRef<[u8]>) -> String {
    bcrypt::hash(input, bcrypt::DEFAULT_COST).unwrap()
}

pub fn bcrypt_verify(pass: impl AsRef<[u8]>, hash: impl AsRef<str>) -> bool {
    bcrypt::verify(pass, hash.as_ref()).unwrap()
}

/// Hash the input and return a formatted hex
pub fn sha2(input: impl AsRef<[u8]>) -> String {
    let mut h = Sha256::new();
    h.update(input);
    let ret = h.finalize();
    format!("{:X}", ret)
}
