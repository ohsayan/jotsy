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
    error::ResponseError,
    templates::{Account, DeleteUI, NoticePage},
    util::{self, resp},
};
use axum::{
    extract::{Extension, Form},
    http::StatusCode,
};
use serde::Deserialize;
use skytable::{
    actions::AsyncActions,
    aio::Connection,
    ddl::AsyncDdl,
    error::{Error, SkyhashError},
    pool::AsyncPool,
    query, Element, RespCode,
};
use tower_cookies::Cookies;

pub async fn account(
    mut cookies: Cookies,
    Extension(db): Extension<AsyncPool>,
) -> crate::JotsyResponse {
    let mut con = db.get().await?;
    let username = super::root::verify_user_or_error(&mut con, &mut cookies).await?;
    con.switch(crate::TABLE_NOTES).await?;
    let q = query!("LGET", &username, "len");
    let count = match con.run_simple_query(&q).await? {
        Element::UnsignedInt(uint) => uint,
        _ => {
            return Err(ResponseError::DatabaseError(
                SkyhashError::UnexpectedDataType.into(),
            ))
        }
    };
    resp(StatusCode::OK, Account::new(count, username))
}

async fn delete(
    what: &'static str,
    path: &'static str,
    lose: &'static str,
    mut cookies: Cookies,
    db: AsyncPool,
) -> crate::JotsyResponse {
    let mut con = db.get().await?;
    let un = super::root::verify_user_or_error(&mut con, &mut cookies).await?;
    resp(StatusCode::OK, DeleteUI::new(what, path, un, lose))
}

pub async fn del_account_get(
    cookies: Cookies,
    Extension(db): Extension<AsyncPool>,
) -> crate::JotsyResponse {
    self::delete(
        "your account",
        "account",
        "your account and all your notes",
        cookies,
        db,
    )
    .await
}

pub async fn del_notes_get(
    cookies: Cookies,
    Extension(db): Extension<AsyncPool>,
) -> crate::JotsyResponse {
    self::delete(
        "all your notes",
        "notes",
        "all your existing notes",
        cookies,
        db,
    )
    .await
}

async fn delete_verify(
    cookies: &mut Cookies,
    con: &mut Connection,
    form: DeleteForm,
) -> crate::JotsyResponseResult<String> {
    let username = super::root::verify_user_or_error(con, cookies).await?;
    con.switch(crate::TABLE_AUTH).await?;
    let hash_from_db: Result<String, Error> = con.get(&username).await;
    match hash_from_db {
        Ok(v) if util::bcrypt_verify(&form.password, &v) => Ok(username),
        Err(Error::SkyError(SkyhashError::Code(RespCode::NotFound))) | Ok(_) => Err(
            ResponseError::Redirect(NoticePage::new("Failed to verify secure action", true)),
        ),
        Err(e) => Err(ResponseError::DatabaseError(e)),
    }
}

#[derive(Deserialize)]
pub struct DeleteForm {
    password: String,
}

pub async fn del_account_post(
    mut cookies: Cookies,
    Extension(db): Extension<AsyncPool>,
    Form(form): Form<DeleteForm>,
) -> crate::JotsyResponse {
    let mut con = db.get().await?;
    let username = self::delete_verify(&mut cookies, &mut con, form).await?;
    // cool, let's first delete the notes (to avoid a new user taking over this user's notes)
    con.switch(crate::TABLE_NOTES).await?;
    con.del(&username).await?;
    // now, let's delete the user token (user -> pass)
    con.switch(crate::TABLE_AUTH).await?;
    con.del(&username).await?;
    drop(con);
    // now log the user out
    log::info!("Deleted account `{username}`");
    super::logout::logout_core(cookies, "Finished deleting account", db).await
}

pub async fn del_notes_post(
    mut cookies: Cookies,
    Extension(db): Extension<AsyncPool>,
    Form(form): Form<DeleteForm>,
) -> crate::JotsyResponse {
    let mut con = db.get().await?;
    let username = self::delete_verify(&mut cookies, &mut con, form).await?;
    con.switch(crate::TABLE_NOTES).await?;
    if let Element::RespCode(RespCode::Okay) = con
        .run_simple_query(&query!("LMOD", username, "clear"))
        .await?
    {
        resp(
            StatusCode::OK,
            NoticePage::new_redirect("Deleted all notes"),
        )
    } else {
        Err(ResponseError::DatabaseError(
            SkyhashError::UnexpectedDataType.into(),
        ))
    }
}
