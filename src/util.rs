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

use skytable::pool::AsyncPool;
use skytable::Query;

const CREATE_JOTSY_TABLE_AUTH: &str = "create table jotsyauth keymap(binstr,binstr)";
const CREATE_JOTSY_TABLE_NOTES: &str = "create table jotsynotes keymap(str,list<str>)";

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
