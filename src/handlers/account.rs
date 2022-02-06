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

use crate::{error::ResponseError, templates::Account, util::resp};
use axum::{extract::Extension, http::StatusCode};
use skytable::{ddl::AsyncDdl, error::SkyhashError, pool::AsyncPool, query, Element};
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
