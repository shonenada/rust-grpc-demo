# Demo

## Prerequisites

```
$ brew install grpc
$ brew install cmake
$ cargo install protobuf
$ cargo install grpc-compiler
```

## Run exmpales

### Run server

```shell
$ cargo run --bin server
```

### Run client

```shell
$ cargo run --bin client localhost 8120
```

or 

```shell
$ cargo run --bin async_client localhost 8120
```
