#[macro_use]
extern crate hyper;
extern crate reqwest;
extern crate protobuf;

mod embedded_assistant;
mod latlng;

use embedded_assistant::*;

use std::str::FromStr;
use reqwest::header::ContentType;
use reqwest::mime::Mime;
use protobuf::parse_from_bytes;
use protobuf::Message;

header! { (Authorization, "Authorization") => [String] }

fn main() {
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
  config.set_text_query("what's the time".to_owned());
  config.set_audio_out_config(audio_out_config);
  config.set_dialog_state_in(dialog_state);
  config.set_device_config(device_config);
  config.set_screen_out_config(screen_out_config);
  let mut req = AssistRequest::new();
  req.set_config(config);
  println!("sending request {:#?}", req);
  let serialized_bytes = req.write_to_bytes().unwrap();

  let client = reqwest::Client::new();
  let mut res = client.post("https://embeddedassistant.googleapis.com/$rpc/google.assistant.embedded.v1alpha2.EmbeddedAssistant/Assist")
    .header(ContentType(Mime::from_str("application/x-protobuf").unwrap()))
    .header(Authorization("Bearer blah".to_owned()))
    .body(serialized_bytes)
    .send().unwrap();

  println!("Status: {}", res.status());
  println!("Headers:\n{}", res.headers());

  let mut buf: Vec<u8> = vec![];
  res.copy_to(&mut buf).unwrap();
  let body_str = std::str::from_utf8(&buf);
  println!("body: {:?}", body_str);
  let entities = parse_from_bytes::<AssistResponse>(&buf).unwrap();
  println!("RESULTS\n{:?}", &entities);
}
