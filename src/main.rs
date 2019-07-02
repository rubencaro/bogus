extern crate hyper;

use hyper::rt::{self, Future};
use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, Server, StatusCode};
use std::env;

fn main() {
    let (port, status) = parse_args();
    let addr = ([0, 0, 0, 0], port).into();

    let server = Server::bind(&addr)
        .serve(move || {
            service_fn_ok(move |_req: Request<Body>| {
                // println!("{}", req.uri().path());
                let mut response = Response::new(Body::from("OK"));
                *response.status_mut() = status;
                response
            })
        })
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}

fn parse_args() -> (u16, StatusCode) {
    let mut args = env::args();
    args.next(); // the command itself
    (parse_port(&mut args), parse_status(&mut args))
}

fn parse_port(args: &mut env::Args) -> u16 {
    match args.next() {
        Some(arg) => parse_u16(arg, 3000),
        None => 3000,
    }
}

fn parse_code(args: &mut env::Args) -> u16 {
    match args.next() {
        Some(arg) => parse_u16(arg, 200),
        None => 200,
    }
}

fn parse_status(args: &mut env::Args) -> StatusCode {
    match StatusCode::from_u16(parse_code(args)) {
        Ok(st) => st,
        Err(_) => StatusCode::OK,
    }
}

fn parse_u16(arg: String, default: u16) -> u16 {
    match arg.parse::<u16>() {
        Ok(p) => p,
        Err(_) => default,
    }
}