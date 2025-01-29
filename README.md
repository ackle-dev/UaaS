# UaaS: UwU as a Service

It's what it sounds like. The [fastest text uwuifier in the west](https://github.com/Daniel-Liu-c0deb0t/uwu)... as a *service*.

(It's not *really* an XaaS, but it runs an HTTP server and returns uwuified text, so close enough.)

There's an [uwuified version of this README](./WEADME.md). I'm too lazy to preserve the shell commands and links. Besides, it's funnier this way.

## Installation:

Install from crates.io:
```bash
cargo install UaaS
uaas
```

Or clone and run:
```bash
git clone https://github.com/ackle-dev/UaaS.git && cd UaaS
cargo run
```

## Usage:

Environment variable `PORT` can be set to change the port the server runs on. By default, it runs on port 41235, because why not?

```bash
PORT=8001 uaas # if installed via cargo
PORT=8001 cargo run # if cloned
```

Then, you can uwuify text by sending a PUT request to `http://localhost:41235/` (or whatever port you choose) with the plaintext in the body (not JSON!).

For example, using `curl`:
```bash
curl -X PUT -d "hello world" http://localhost:41235/ # returns "hewwo wowwd"
```

## How's it work?

I've got no clue, I just hooked up [the `uwuify` crate](https://crates.io/crates/uwuify) to an HTTP server.

## This code sucks.

Yeah, probably. I barely know any Rust. You can improve it with a PR if you'd like.
