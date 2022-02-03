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

use axum::{
    http::header::{self, HeaderMap, HeaderValue},
    response::IntoResponse,
};

const CSS_INDEX_LOGIN: &str = include_str!("../../static/css/login.css");
const JS_INDEX_LOGIN: &str = include_str!("../../static/js/login.js");
const JS_INDEX_APP: &str = include_str!("../../static/js/app.js");

async fn asset(source: &'static str, ty: &'static str) -> impl IntoResponse {
    let mut headermap = HeaderMap::new();
    headermap.insert(header::CONTENT_TYPE, HeaderValue::from_static(ty));
    (headermap, source)
}

async fn css(source: &'static str) -> impl IntoResponse {
    asset(source, "text/css").await
}

async fn js(source: &'static str) -> impl IntoResponse {
    asset(source, "text/javascript").await
}

pub async fn index_login_css() -> impl IntoResponse {
    css(CSS_INDEX_LOGIN).await
}

pub async fn index_login_js() -> impl IntoResponse {
    js(JS_INDEX_LOGIN).await
}

pub async fn index_app_js() -> impl IntoResponse {
    js(JS_INDEX_APP).await
}
