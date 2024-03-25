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
mod radio;

use crate::{
    assistant::{get_token, make_request},
    config::Config,
};
use eyre::Report;
use log::{error, info, warn};
use radio::Radio;
use rfbutton::{decode, Code};

#[tokio::main]
async fn main() -> Result<(), Report> {
    pretty_env_logger::init();

    let config = Config::from_file()?;

    let mut radio = Radio::init()?;

    loop {
        match radio.receive() {
            Ok(pulses) => {
                if pulses.len() > 10 {
                    info!("{} pulses: {:?}...", pulses.len(), &pulses[0..10]);
                } else {
                    info!("{} pulses: {:?}", pulses.len(), pulses);
                }
                match decode(&pulses) {
                    Ok(code) => {
                        if code.length > 0 {
                            info!("Decoded: {:?}", code);
                            handle_code(&config, code).await?;
                        } else {
                            warn!("Decoded 0 bits.");
                        }
                    }
                    Err(e) => {
                        error!("Decode error: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("Receive error: {}", e);
            }
        }
    }
}

async fn handle_code(config: &Config, code: Code) -> Result<(), Report> {
    if let Some(command) = config.commands.get(&code) {
        let token = get_token(config).await?;
        make_request(config, &token, command).await?;
    }
    Ok(())
}
