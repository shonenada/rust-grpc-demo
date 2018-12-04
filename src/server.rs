extern crate futures;
extern crate grpcio;
extern crate protos;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use crate::protos::echo::{EchoRequest, EchoResponse};
use crate::protos::echo_grpc::{self, Echo};

#[derive(Clone)]
struct EchoService;

impl Echo for EchoService {
    fn echo(&mut self, ctx: RpcContext, req: EchoRequest, sink: UnarySink<EchoResponse>) {
        let echo = format!("Echo {}", req.get_echo());
        let mut resp = EchoResponse::new();
        resp.set_success(true);
        resp.set_echo(echo.clone());
        let f = sink
            .success(resp.clone())
            .map(move |_| println!("Response with {:?}", echo))
            .map_err(move |err| eprintln!("Failed to reply {:?}", err));
        ctx.spawn(f)
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let echo_service = echo_grpc::create_echo(EchoService);
    let mut server = ServerBuilder::new(env)
        .register_service(echo_service)
        .bind("127.0.0.1", 8120)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
