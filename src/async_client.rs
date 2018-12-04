extern crate protos;

use std::env;
use std::sync::Arc;

use tokio::prelude::*;
use grpcio::{ChannelBuilder, EnvBuilder};

use crate::protos::echo::EchoRequest;
use crate::protos::echo_grpc::EchoClient;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        panic!("Please input server host and port")
    }
    let port = args[2]
        .parse::<u16>()
        .expect(format!("{} is not a number", args[2]).as_str());

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(format!("{}:{}", args[1], port).as_str());
    let client = EchoClient::new(ch);

    let mut request = EchoRequest::new();
    request.set_echo("foo".to_string());

    let f = client.echo_async(&request)
        .expect("RPC Failed!")
        .map(|resp| println!("Async Pong: {}", resp.get_echo()))
        .map_err(|e| eprintln!("Err: {}", e));

    tokio::run(f)
}
