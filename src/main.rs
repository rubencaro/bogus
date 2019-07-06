#[macro_use]
extern crate clap;
extern crate hyper;
use hyper::rt::{self, Future};
use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, Server, StatusCode};

fn main() {
    let matches = clap::App::new("Bogus")
        .version("0.1.0")
        .about("Bogus server that only responds, doing nothing else.")
        .args_from_usage(
            "-p, --port [number] 'Sets a custom port to listen to'
             -c, --code [code] 'Sets a custom response code'",
        )
        .get_matches();

    let port = parse_u16(&matches, "port", 3000);
    let addr = ([0, 0, 0, 0], port).into();
    let status = parse_status(&matches, 200);

    let server = Server::bind(&addr)
        .serve(move || {
            service_fn_ok(move |_req: Request<Body>| {
                let mut response = Response::new(Body::from("OK"));
                *response.status_mut() = status;
                response
            })
        })
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}

fn parse_u16(matches: &clap::ArgMatches<'_>, name: &str, default: u16) -> u16 {
    value_t!(matches, name, u16).unwrap_or_else(|e| {
        println!("{}\nUsing default one.", e);
        default
    })
}

fn parse_status(matches: &clap::ArgMatches<'_>, default: u16) -> StatusCode {
    match StatusCode::from_u16(parse_u16(&matches, "code", default)) {
        Ok(st) => st,
        Err(e) => {
            println!("Not a valid status code: {}\nUsing 200.", e);
            StatusCode::OK
        },
    }
}
