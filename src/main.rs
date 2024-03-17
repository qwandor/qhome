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

#[macro_use]
extern crate hyper;
extern crate protobuf;
extern crate reqwest;

mod embedded_assistant;
mod latlng;
mod stream_body;

use embedded_assistant::*;
use stream_body::StreamBody;

use protobuf::parse_from_bytes;
use protobuf::Message;
use reqwest::header::ContentType;
use reqwest::mime::Mime;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;

header! { (Authorization, "Authorization") => [String] }

fn make_request() -> Vec<u8> {
    let mut audio_out_config = AudioOutConfig::new();
    audio_out_config.encoding = AudioOutConfig_Encoding::MP3;
    audio_out_config.sample_rate_hertz = 16000;
    //audio_out_config.volume_percentage = 50;
    let mut dialog_state = DialogStateIn::new();
    dialog_state.language_code = "en-US".to_owned();
    let mut device_config = DeviceConfig::new();
    device_config.device_id = "my_device_id".to_owned();
    device_config.device_model_id = "qhome-887f8-qhome-button-ty2jrt".to_owned();
    let mut audio_in_config = AudioInConfig::new();
    audio_in_config.encoding = AudioInConfig_Encoding::LINEAR16;
    audio_in_config.sample_rate_hertz = 16000;
    let mut screen_out_config = ScreenOutConfig::new();
    screen_out_config.screen_mode = ScreenOutConfig_ScreenMode::PLAYING;
    let mut config = AssistConfig::new();
    //config.set_audio_in_config(audio_in_config);
    config.set_text_query("rainbow lights on for 10 seconds".to_owned());
    config.set_audio_out_config(audio_out_config);
    config.set_dialog_state_in(dialog_state);
    config.set_device_config(device_config);
    config.set_screen_out_config(screen_out_config);
    let mut req = AssistRequest::new();
    req.set_config(config);
    println!("request {:#?}", req);
    let req_bytes = req.write_to_bytes().unwrap();
    let mut stream_body = StreamBody::new();
    stream_body.message.push(req_bytes);
    stream_body.write_to_bytes().unwrap()
}

fn main() {
    let body_bytes = make_request();

    let mut file = File::create("request.pb").unwrap();
    file.write_all(&body_bytes).unwrap();

    let client = reqwest::Client::new();
    let mut res = client.post("https://embeddedassistant.googleapis.com/$rpc/google.assistant.embedded.v1alpha2.EmbeddedAssistant/Assist")
    .header(ContentType(Mime::from_str("application/x-protobuf").unwrap()))
    .header(Authorization("Bearer blah".to_owned()))
    .body(body_bytes)
    .send().unwrap();

    println!("Status: {}", res.status());
    println!("Headers:\n{}", res.headers());

    let mut buf: Vec<u8> = vec![];
    res.copy_to(&mut buf).unwrap();
    let stream_response = parse_from_bytes::<StreamBody>(&buf).unwrap();
    println!(
        "status: {:?} messages: {}",
        &stream_response.status,
        &stream_response.message.len()
    );
    for message in stream_response.message.iter() {
        let response = parse_from_bytes::<AssistResponse>(&message).unwrap();
        println!("response: {:?}", response);
    }
}
