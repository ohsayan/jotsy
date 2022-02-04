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

pub mod app;
pub mod assets;
mod login;
mod logout;
mod root;
pub mod signup;

pub use self::{
    login::{login, login_get},
    logout::logout,
    root::root,
    signup::{signup, signup_get},
};

use crate::templates::NoticePage;
use axum::response::Html;
use tower_cookies::Cookies;
const COOKIE_USERNAME: &str = "jotsy_user";
const COOKIE_TOKEN: &str = "jotsy_token";

async fn redirect_home_if_cookie_set(cookies: Cookies, page: String) -> Html<String> {
    if cookies.get(COOKIE_TOKEN).is_some() || cookies.get(COOKIE_USERNAME).is_some() {
        // someone set the cookies but still ended up here, so redirect them to root to handle
        // the login cookie state
        return Html::from(NoticePage::empty());
    } else {
        Html::from(page)
    }
}
