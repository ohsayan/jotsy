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

use crate::util::resp;
use axum::{http::StatusCode, response::Html};
use skytable::pool::AsyncPool;

pub async fn app(uname: String, _db: AsyncPool) -> (StatusCode, Html<String>) {
    resp(
        StatusCode::OK,
        format!(
            r#"
            <html>
            <h1>Hey, {uname}! Welcome to Jotsy &mdash; it's just around the corner</h1>
            <form action="/logout" method="post">
            <input type="submit" value="Click to Logout">
            </form> 
            </html>
            "#
        ),
    )
}
