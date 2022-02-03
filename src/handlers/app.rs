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
    templates::{App, RedirectHome},
    util::resp,
};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use skytable::{ddl::AsyncDdl, pool::AsyncPool, query, types::Array, Element};

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub date: String,
    pub body: String,
}

pub async fn app(uname: String, db: AsyncPool) -> crate::RespTuple {
    let mut con = match db.get().await {
        Ok(c) => c,
        Err(e) => {
            log::error!("Failed to get connection from pool: {e}");
            return resp(StatusCode::INTERNAL_SERVER_ERROR, RedirectHome::e500());
        }
    };
    con.switch(crate::TABLE_NOTES).await.unwrap();
    let query = query!("LGET", &uname);
    let notes: Vec<Note> =
        if let Element::Array(Array::Str(e)) = con.run_simple_query(&query).await.unwrap() {
            e.into_iter()
                .rev()
                .filter_map(|v| v.map(|v| serde_json::from_str(&v).unwrap()))
                .collect()
        } else {
            return resp(StatusCode::INTERNAL_SERVER_ERROR, RedirectHome::e500());
        };
    resp(StatusCode::OK, App::new(uname, notes))
}
