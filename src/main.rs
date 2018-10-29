extern crate grpcio;
extern crate futures;
extern crate protobuf;
extern crate tls_api_native_tls;
extern crate httpbis;
extern crate tls_api;

mod embedded_assistant;
mod embedded_assistant_grpc;
mod latlng;

use std::sync::Arc;
use std::fs;
use grpcio::*;
use embedded_assistant::*;
use embedded_assistant_grpc::*;
use futures::future;
use futures::future::Future;
use futures::Stream;
use futures::Sink;

fn main() {
  let env = Arc::new(Environment::new(2));
  let root_cert = fs::read("/home/qwandor/projects/qhome/gtsr2.pem").expect("tls.cert file not found");
  let credentials = ChannelCredentialsBuilder::new().root_cert(root_cert).build();
  let channel = ChannelBuilder::new(env).secure_connect("embeddedassistant.googleapis.com:443", credentials);
  let client = EmbeddedAssistantClient::new(channel);
  
  let (mut sink, mut receiver) = client.assist().unwrap();
  let req = AssistRequest::new();
  println!("Sending request {:?}", req);
  sink = sink.send((req, WriteFlags::default())).wait().unwrap();
  println!("Sent");
  future::poll_fn(|| sink.close()).wait().unwrap();
  println!("Flushed");

  match receiver.into_future().wait() {
    Ok((Some(response), r)) => {
      println!("response: {:?}", response);
    },
    Ok((None, _)) => {
      println!("no response");
    },
    Err((e, _)) => panic!("error {:?}", e),
  }
}
