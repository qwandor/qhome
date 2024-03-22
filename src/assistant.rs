// Copyright 2024 Google LLC
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

use crate::config::Config;
use eyre::Report;
use googapis::{
    google::assistant::embedded::v1alpha2::{
        assist_config, assist_request, audio_out_config::Encoding,
        embedded_assistant_client::EmbeddedAssistantClient, AssistConfig, AssistRequest,
        AudioOutConfig, DeviceConfig,
    },
    CERTIFICATES,
};
use log::trace;
use oauth2::{
    reqwest::async_http_client,
    {AuthUrl, ClientId, ClientSecret, RefreshToken, TokenResponse, TokenUrl},
};
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
    Request,
};

const OAUTH_AUTH_URL: &str = "https://oauth2.googleapis.com/auth";
const OAUTH_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const ASSISTANT_API_HOSTNAME: &str = "embeddedassistant.googleapis.com";
const ASSISTANT_API_URL: &str = "https://embeddedassistant.googleapis.com";

const DEVICE_ID: &str = "my_device_id";
const DEVICE_MODEL_ID: &str = "qhome-887f8-qhome-button-ty2jrt";

pub async fn get_token(config: &Config) -> Result<String, Report> {
    let client = oauth2::basic::BasicClient::new(
        ClientId::new(config.client_id.to_string()),
        Some(ClientSecret::new(config.client_secret.to_string())),
        AuthUrl::new(OAUTH_AUTH_URL.to_string())?,
        Some(TokenUrl::new(OAUTH_TOKEN_URL.to_string())?),
    );

    let token_response = client
        .exchange_refresh_token(&RefreshToken::new(config.refresh_token.to_string()))
        .request_async(async_http_client)
        .await?;

    Ok(token_response.access_token().secret().clone())
}

pub async fn make_request(bearer: &str, command: &str) -> Result<(), Report> {
    let tls_config = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(CERTIFICATES))
        .domain_name(ASSISTANT_API_HOSTNAME);

    let channel = Channel::from_static(ASSISTANT_API_URL)
        .tls_config(tls_config)?
        .connect()
        .await?;

    let mut service =
        EmbeddedAssistantClient::with_interceptor(channel, move |mut req: Request<()>| {
            let meta = MetadataValue::from_str(&format!("Bearer {}", bearer)).unwrap();
            req.metadata_mut().insert("authorization", meta);
            Ok(req)
        });

    let config = AssistConfig {
        r#type: Some(assist_config::Type::TextQuery(command.to_string())),
        audio_out_config: Some(AudioOutConfig {
            encoding: Encoding::Mp3.into(),
            sample_rate_hertz: 16000,
            volume_percentage: 50,
        }),
        device_config: Some(DeviceConfig {
            device_id: DEVICE_ID.to_owned(),
            device_model_id: DEVICE_MODEL_ID.to_owned(),
        }),
        ..Default::default()
    };
    let request = AssistRequest {
        r#type: Some(assist_request::Type::Config(config)),
    };
    let request = futures::stream::once(async move { request });
    let response = service.assist(request).await?;

    trace!("Response: {:?}", response);
    if let Some(msg) = response.into_inner().message().await? {
        trace!("Response message: {:?}", msg);
    }

    Ok(())
}
