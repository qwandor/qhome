extern crate grpc;
extern crate futures;
extern crate protobuf;
extern crate tls_api_native_tls;

mod embedded_assistant;
mod embedded_assistant_grpc;
mod latlng;

use std::sync::Arc;
use grpc::ClientStub;
use grpc::StreamingRequest;
use embedded_assistant::*;
use embedded_assistant_grpc::*;
use futures::future::Future;

fn main() {
  let port = 50052;
  let client_conf = Default::default();

  let grpc_client = Arc::new(grpc::Client::new_tls::<tls_api_native_tls::TlsConnector>("embeddedassistant.googleapis.com", port, client_conf).unwrap());
  let client = EmbeddedAssistantClient::with_client(grpc_client);
  
  let mut req = AssistRequest::new();
  println!("Sending request {:?}", req);
  let resp = client.assist(grpc::RequestOptions::new(), StreamingRequest::once(req));
  println!("Sent");
  let (metadata, stream) = resp.0.wait().unwrap();
  println!("metadata: {:?}", metadata);
  //for item in stream {
  //  println!("{:?}", item);
  //}
}
