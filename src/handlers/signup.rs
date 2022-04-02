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
    templates::{NoticePage, SignupPage},
    util::{self, resp},
};
use axum::{
    extract::{Extension, Form},
    http::StatusCode,
    response::Html,
};
use serde::Deserialize;
use skytable::{
    actions::AsyncActions, ddl::AsyncDdl, error::SkyhashError, pool::AsyncPool, query, Element,
    RespCode,
};
use tower_cookies::Cookies;

#[derive(Deserialize)]
/// The sign up form
pub struct SignupForm {
    username: String,
    password: String,
    vpassword: String,
}

/// `GET` for `/signup`
/// If no cookies are set, simply return a fresh sign up page; else, reload `/` to trigger
/// auth
pub async fn signup_get(cookies: Cookies) -> Html<String> {
    super::redirect_home_if_cookie_set(cookies, SignupPage::empty()).await
}

/// `POST` for `/signup`
///
/// Signup flow:
/// 1. Hash the password (TODO: report error if vpassword != password)
/// 2. Now `set` username->hashed passowrd
/// a. If this fails, username is taken
/// b. If this succeeds, username is available and we've created an user
/// 3. Now call super::login::authenticate(username, &mut cookies, &mut connection)
///
pub async fn signup(
    Form(data): Form<SignupForm>,
    mut cookies: Cookies,
    Extension(db): Extension<AsyncPool>,
) -> crate::JotsyResponse {
    // do a double check on the data; never trust the client
    if data.username.len() < 6 {
        return resp(
            StatusCode::UNPROCESSABLE_ENTITY,
            SignupPage::render_new("Username must have atleast 6 letters"),
        );
    }
    if data.username.chars().any(|ch| !ch.is_ascii_alphanumeric()) {
        // some funky chars in the username; let's prevent that
        return resp(
            StatusCode::UNPROCESSABLE_ENTITY,
            SignupPage::render_new("Username can only have alphanumeric characters"),
        );
    }
    if data.password != data.vpassword {
        return resp(
            StatusCode::UNPROCESSABLE_ENTITY,
            SignupPage::render_new("The passwords do not match"),
        );
    }
    if data.password.len() < 8 {
        return resp(
            StatusCode::UNPROCESSABLE_ENTITY,
            SignupPage::render_new("Passwords must have atleast 8 characters"),
        );
    }
    let hash = util::bcrypt_hash(&data.password);
    let mut con = db.get().await?;
    con.switch(crate::TABLE_AUTH).await?;
    match con.set(data.username.clone(), hash).await {
        Ok(created_new) if created_new => {
            // cool, we did well
            log::info!("New user `{uname}` created.", uname = data.username);
            let ret =
                super::login::authenticate(data.username.clone(), &mut cookies, &mut con).await?;
            con.switch(crate::TABLE_NOTES).await?;
            // attempt to create an empty list
            let query = query!("LSET", data.username);
            let create_empty_result = con.run_simple_query(&query).await?;
            if let Element::RespCode(RespCode::Okay) = create_empty_result {
                Ok(ret)
            } else {
                Err(ResponseError::DatabaseError(
                    SkyhashError::UnexpectedDataType.into(),
                ))
            }
        }
        Ok(_) => {
            // nope, username is taken
            resp(
                StatusCode::CONFLICT,
                SignupPage::render_new("Sorry, that username is taken"),
            )
        }
        Err(e) => {
            // server error
            log::error!("Failed to create user: {e}");
            NoticePage::re500()
        }
    }
}

/// `GET` for `/signup` for cases where signups are disabled
pub async fn no_signup() -> crate::JotsyResponse {
    resp(
        StatusCode::BAD_REQUEST,
        NoticePage::render_new(
            "Signups are currently disabled on this Jotsy instance",
            false,
        ),
    )
}
