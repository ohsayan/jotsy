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
    templates::{App, RedirectHome},
    util::resp,
};
use axum::{
    extract::{Extension, Form},
    http::StatusCode,
};
use chrono::prelude::Local;
use serde::{Deserialize, Serialize};
use skytable::{ddl::AsyncDdl, pool::AsyncPool, query, types::Array, Element, RespCode};
use tower_cookies::Cookies;

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub date: String,
    pub body: String,
}

impl Note {
    fn new(date: String, body: String) -> Self {
        Self { date, body }
    }
}

pub async fn app(uname: String, db: AsyncPool) -> crate::RespTuple {
    let mut con = match db.get().await {
        Ok(c) => c,
        Err(e) => {
            log::error!("Failed to get connection from pool: {e}");
            return RedirectHome::re500();
        }
    };
    con.switch(crate::TABLE_NOTES).await.unwrap();
    let query = query!("LGET", &uname);
    let ret = con.run_simple_query(&query).await.unwrap();
    let notes: Vec<Note> = if let Element::Array(Array::Str(e)) = ret {
        e.into_iter()
            .rev()
            .filter_map(|v| v.map(|v| serde_json::from_str(&v).unwrap()))
            .collect()
    } else {
        log::error!("Failed to LGET notes");
        return RedirectHome::re500();
    };
    resp(StatusCode::OK, App::new(uname, notes))
}

#[derive(Deserialize)]
pub struct FormNote {
    note: String,
}

pub async fn create_note(
    mut cookies: Cookies,
    Extension(db): Extension<AsyncPool>,
    Form(note): Form<FormNote>,
) -> crate::RespTuple {
    let time = Local::now().format("%B %d, %Y | %I:%S %p").to_string();
    let mut con = match db.get().await {
        Ok(c) => c,
        Err(e) => {
            log::error!("Failed to get connection from pool: {e}");
            return RedirectHome::re500();
        }
    };
    // verify the user
    let ret = super::root::verify_user_or_error(&mut con, &mut cookies).await;
    let username = match ret {
        Ok(uname) => uname,
        Err(e) => return e,
    };
    // now create the note
    con.switch(crate::TABLE_NOTES).await.unwrap();
    let json = serde_json::to_string(&Note::new(time, note.note)).unwrap();
    let query = query!("LMOD", &username, "PUSH", json);
    match con.run_simple_query(&query).await {
        Ok(Element::RespCode(RespCode::Okay)) => resp(
            StatusCode::CREATED,
            RedirectHome::new("Created note successfully"),
        ),
        Ok(_) => RedirectHome::re500(),
        Err(e) => {
            log::error!("Error while creating note: {e}");
            RedirectHome::re500()
        }
    }
}
