// Copyright 2018 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod assistant;
mod config;

use crate::{
    assistant::{get_token, make_request},
    config::Config,
};
use eyre::Report;

#[tokio::main]
async fn main() -> Result<(), Report> {
    pretty_env_logger::init();

    let config = Config::from_file()?;
    let token = get_token(&config).await?;
    make_request(&token, "bedside lamp on").await?;
    make_request(&token, "bedside lamp off").await?;
    Ok(())
}
