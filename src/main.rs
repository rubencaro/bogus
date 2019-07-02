extern crate hyper;

use hyper::rt::{self, Future};
use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, Server};
use std::env;

fn main() {
    let port = parse_port();
    let addr = ([0, 0, 0, 0], port).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn_ok(move |_req: Request<Body>| {
            // println!("{}", req.uri().path());
            Response::new(Body::from("OK"))
            }))
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}

fn parse_port() -> u16 {
    let mut args = env::args();
    args.next(); // the command itself
    match args.next() { // the first argument
        Some(arg) => match arg.parse::<u16>() {
            Ok(p) => p,
            Err(_) => 3000,
        },
        None => 3000,
    }
}