// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_EMBEDDED_ASSISTANT_ASSIST: ::grpcio::Method<super::embedded_assistant::AssistRequest, super::embedded_assistant::AssistResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/google.assistant.embedded.v1alpha2.EmbeddedAssistant/Assist",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct EmbeddedAssistantClient {
    client: ::grpcio::Client,
}

impl EmbeddedAssistantClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        EmbeddedAssistantClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn assist_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::embedded_assistant::AssistRequest>, ::grpcio::ClientDuplexReceiver<super::embedded_assistant::AssistResponse>)> {
        self.client.duplex_streaming(&METHOD_EMBEDDED_ASSISTANT_ASSIST, opt)
    }

    pub fn assist(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::embedded_assistant::AssistRequest>, ::grpcio::ClientDuplexReceiver<super::embedded_assistant::AssistResponse>)> {
        self.assist_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait EmbeddedAssistant {
    fn assist(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::embedded_assistant::AssistRequest>, sink: ::grpcio::DuplexSink<super::embedded_assistant::AssistResponse>);
}

pub fn create_embedded_assistant<S: EmbeddedAssistant + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_EMBEDDED_ASSISTANT_ASSIST, move |ctx, req, resp| {
        instance.assist(ctx, req, resp)
    });
    builder.build()
}
