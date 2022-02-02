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

use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginPage {
    login_failed: bool,
}

impl LoginPage {
    pub fn new(login_failed: bool) -> String {
        Self { login_failed }.render().unwrap()
    }
}

#[derive(Template)]
#[template(path = "redirect.html")]
pub struct RedirectHome {
    message: String,
}

impl RedirectHome {
    pub fn new(message: impl ToString) -> String {
        RedirectHome {
            message: message.to_string(),
        }
        .render()
        .unwrap()
    }
    pub fn e500() -> String {
        RedirectHome {
            message: "An internal server error occurred.".to_owned(),
        }
        .render()
        .unwrap()
    }
}

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignupPage {
    conflict: bool,
}

impl SignupPage {
    pub fn new(conflict: bool) -> String {
        Self { conflict }.render().unwrap()
    }
}
