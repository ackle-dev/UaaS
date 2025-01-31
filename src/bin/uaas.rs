use tiny_http::{Header, Method, Request, Response, Server};
use uwuifier::uwuify_str_sse;

fn main() {
    let port = match std::env::var("PORT") {
        Ok(port) => port,
        Err(_) => 41235.to_string(),
    };
    let server = Server::http(format!("0.0.0.0:{}", port)).unwrap();

    println!("sewvew stawted on powt {} uwu", port);

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
            log_bad_request(&mut req);
            req.respond(
                Response::from_string(
                    "unknown w-woute. s-sewvew onwy handwes put wequests to `/`. σωσ",
                )
                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap())
                .with_header(
                    Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap(),
                )
                .with_status_code(400),
            )
            .unwrap();
            continue;
        }
        if req.method() != &Method::Put {
            log_bad_request(&mut req);
            req.respond(
                Response::from_string(
                    "invawid wequest method. rawr x3 s-sewvew onwy h-handwes put wequests t-to `/` :3",
                )
                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap())
                .with_header(
                    Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap(),
                )
                .with_status_code(400),
            )
            .unwrap();
            continue;
        }

        let mut buf = String::new();
        req.as_reader().read_to_string(&mut buf).unwrap();

        if buf.is_empty() {
            log_bad_request(&mut req);
            req.respond(
                Response::from_string(
                    "empty w-wequest body. σωσ p-pwease pwovide a-a stwing to uwuify nyaa~",
                )
                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap())
                .with_header(
                    Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap(),
                )
                .with_status_code(400),
            )
            .unwrap();
            continue;
        }

        req.respond(
            Response::from_string(uwuify_str_sse(&buf))
                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap())
                .with_header(
                    Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap(),
                )
                .with_status_code(200),
        )
        .inspect_err(|e| eprintln!("{}", e))
        .unwrap();
    }
}

fn log_bad_request(req: &mut Request) {
    let mut buf = String::new();
    req.as_reader().read_to_string(&mut buf).unwrap();

    println!(
        "bad {} wequest fwom {}: {} {}",
        req.method(),
        req.remote_addr().unwrap(),
        req.url(),
        buf
    );
}
