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

use crate::templates::NoticePage;
use axum::{
    body,
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
pub use skytable::{error::Error as SkytableError, pool::bb8Error};

#[derive(Debug)]
pub enum ResponseError {
    DatabaseError(SkytableError),
    PoolError(bb8Error<SkytableError>),
    /// This is a redirect, not an error. Just a hack to simplify things
    Redirect(String),
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        let mut r = match self {
            Self::DatabaseError(dbe) => {
                log::error!("Database error: {dbe}");
                NoticePage::e500_resp()
            }
            Self::PoolError(epool) => {
                log::error!("Failed to get connection from pool: {epool}");
                NoticePage::e500_resp()
            }
            Self::Redirect(red) => Response::builder()
                .status(StatusCode::OK)
                .body(body::boxed(body::Full::from(red)))
                .unwrap(),
        };
        r.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::TEXT_HTML_UTF_8.as_ref()),
        );
        r
    }
}

impl From<SkytableError> for ResponseError {
    fn from(e: SkytableError) -> Self {
        Self::DatabaseError(e)
    }
}

impl From<bb8Error<SkytableError>> for ResponseError {
    fn from(e: bb8Error<SkytableError>) -> Self {
        Self::PoolError(e)
    }
}
