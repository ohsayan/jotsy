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

use envconfig::{Envconfig, Error};

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "JOTSY_SKY_HOST", default = "127.0.0.1")]
    pub sky_host: String,
    #[envconfig(from = "JOTSY_SKY_PORT", default = "2003")]
    pub sky_port: u16,
    #[envconfig(from = "JOTSY_HOST", default = "127.0.0.1")]
    pub host: String,
    #[envconfig(from = "JOTSY_PORT", default = "2022")]
    pub port: u16,
    #[envconfig(from = "JOTSY_SIGNUP_ENABLED", default = "true")]
    pub signup_enabled: bool,
    #[envconfig(from = "JOTSY_DEPLOY_PROD", default = "true")]
    pub is_prod: bool,
}

impl Config {
    pub fn init() -> Result<Self, Error> {
        Envconfig::init_from_env()
    }
}
