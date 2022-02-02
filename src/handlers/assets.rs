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

use crate::templates;
use axum::{
    http::header::{self, HeaderMap, HeaderValue},
    response::IntoResponse,
};

async fn css(source: &'static str) -> impl IntoResponse {
    let mut headermap = HeaderMap::new();
    headermap.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/css"));
    (headermap, source)
}

pub async fn index_login() -> impl IntoResponse {
    css(templates::CSS_INDEX_LOGIN).await
}
