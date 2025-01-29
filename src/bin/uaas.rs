use tiny_http::{Method, Response, Server};
use uwuifier::uwuify_str_sse;

fn main() {
    let server = Server::http("0.0.0.0:41235").unwrap();

    loop {
        let mut req = match server.try_recv() {
            Ok(rq) => match rq.is_none() {
                true => continue,
                false => rq.unwrap(),
            },
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        };

        if req.url() != "/" {
            req.respond(
                Response::from_string("Unknown route. Server only handles PUT requests to `/`.")
                    .with_status_code(400),
            )
            .unwrap();
            continue;
        }
        if req.method() != &Method::Put {
            req.respond(
                Response::from_string(
                    "Invalid request method. Server only handles PUT requests to `/`.",
                )
                .with_status_code(400),
            )
            .unwrap();
            continue;
        }

        let mut buf = String::new();
        req.as_reader().read_to_string(&mut buf).unwrap();
        req.respond(Response::from_string(uwuify_str_sse(&buf)).with_status_code(200))
            .inspect_err(|e| eprintln!("{}", e))
            .unwrap();
    }
}
