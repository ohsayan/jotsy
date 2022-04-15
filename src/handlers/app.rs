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
    templates::{App, NoticePage, SingleNote},
    util::{self, resp},
};
use axum::{
    extract::{Extension, Form},
    http::StatusCode,
};
use chrono::prelude::Local;
use serde::{Deserialize, Serialize};
use skytable::{ddl::AsyncDdl, pool::AsyncPool, query, Element, RespCode};
use tower_cookies::Cookies;

#[derive(Serialize, Deserialize)]
/// A `Note`. This is stored as JSON in Skytable and is ser/de-d as required
pub struct Note {
    pub date: String,
    pub body: String,
}

impl Note {
    fn new_from_json<T: AsRef<str>>(json: T) -> Self {
        let data: Note = serde_json::from_str(json.as_ref()).unwrap();
        Self::new(data.date, util::md_to_html(&data.body))
    }
    fn new(date: String, body: String) -> Self {
        Self { date, body }
    }
}

/// Returns the main app page for an authenticated user
pub async fn app(uname: String, db: AsyncPool) -> crate::JotsyResponse {
    let mut con = db.get().await?;
    con.switch(crate::TABLE_NOTES).await?;
    let query = query!("LGET", &uname);
    let notes: Vec<String> = con.run_query(&query).await?;
    let notes: Vec<Note> = notes.iter().map(Note::new_from_json).collect();
    resp(StatusCode::OK, App::render_new(uname, notes))
}

#[derive(Deserialize)]
/// A note from the AJAX submission
pub struct FormNote {
    note: String,
}

/// `POST` for `/create/note`
///
/// This will:
/// - Verify the session
/// - Create the note
/// - Return a rendered note element
pub async fn create_note(
    mut cookies: Cookies,
    Extension(db): Extension<AsyncPool>,
    Form(note): Form<FormNote>,
) -> crate::JotsyResponse {
    let time = Local::now().format("%B %d, %Y | %I:%M %p").to_string();
    let mut con = db.get().await?;
    // verify the user
    let username = super::root::verify_user_or_error(&mut con, &mut cookies).await?;
    // now create the note
    let note = Note::new(time, note.note);
    con.switch(crate::TABLE_NOTES).await?;
    let json = serde_json::to_string(&note).unwrap();
    let query = query!("LMOD", &username, "PUSH", json);
    match con.run_query(&query).await {
        Ok(Element::RespCode(RespCode::Okay)) => {
            resp(StatusCode::CREATED, SingleNote::render_new(note))
        }
        Ok(_) => NoticePage::re500(),
        Err(e) => {
            log::error!("Error while creating note: {e}");
            NoticePage::re500()
        }
    }
}
