extern crate grpc;
extern crate futures;
extern crate protobuf;
extern crate tls_api_native_tls;
extern crate httpbis;
extern crate tls_api;

mod embedded_assistant;
mod embedded_assistant_grpc;
mod latlng;

use std::sync::Arc;
use std::net::ToSocketAddrs;
use grpc::ClientStub;
use grpc::StreamingRequest;
use embedded_assistant::*;
use embedded_assistant_grpc::*;
use futures::future::Future;
use tls_api::TlsConnector;
use tls_api::TlsConnectorBuilder;

fn main() {
  let port = 443;
  let client_conf = Default::default();

  let tls_connector = tls_api_native_tls::TlsConnector::builder().unwrap();
  let tls_option = httpbis::ClientTlsOption::Tls("embeddedassistant.googleapis.com".to_owned(), Arc::new(tls_connector.build().unwrap()));
  let addr = ("embeddedassistant.googleapis.com", port).to_socket_addrs().unwrap().next().unwrap();
  let grpc_client = Arc::new(grpc::Client::new_expl(&addr, "embeddedassistant.googleapis.com", tls_option, client_conf).unwrap());
  let client = EmbeddedAssistantClient::with_client(grpc_client);
  
  let req = AssistRequest::new();
  println!("Sending request {:?}", req);
  let resp = client.assist(grpc::RequestOptions::new(), StreamingRequest::once(req));
  println!("Sent");
  let (metadata, _stream) = resp.0.wait().unwrap();
  println!("metadata: {:?}", metadata);
}
