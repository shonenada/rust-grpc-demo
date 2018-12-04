extern crate protos;

use std::env;
use std::sync::Arc;

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
    let response = client.echo(&request).expect("RPC Failed!");
    println!("Echo Response: {}", response.get_echo());
}
