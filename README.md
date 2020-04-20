# Actix Request Identifier Middleware

![Rust](https://github.com/vbrandl/actix-request-identifier/workflows/Rust/badge.svg)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/vbrandl/merging-iterator/blob/master/LICENSE-MIT)
[![License](https://img.shields.io/badge/license-Apache-green.svg)](https://github.com/vbrandl/merging-iterator/blob/master/LICENSE-APACHE)

`actix-request-identifier` implements a middleware for
[`actix-web`](https://github.com/actix/actix-web) that generates an ID for each
incomming request. By default, a UUID v4 is generated for each request using
[`uuid`](https://github.com/uuid-rs/uuid). The request ID is set via the
`x-request-id` HTTP header (which can be configured) and can be extracted from
the request using `RequestId`.

## Example

```rust
use actix_request_identifier::{RequestId, RequestIdentifier};

#[get("/")]
async fn index(id: RequestId) -> impl Responder {
    HttpResponse::Ok().body(format!("{}", id.as_str()))
}

#[actix_rt::main]
async fn main() {
    HttpServer::new(move || {
        App::new()
            .wrap(RequestIdentifier::with_uuid())
            .service(index)
    }).bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
    .unwrap();
```

The response looks like this:

```
$ curl -i 127.0.0.1:8080/
HTTP/1.1 200 OK
content-length: 36
x-request-id: 5f099854-2117-49b3-b252-d6693a85acc5
date: Mon, 20 Apr 2020 06:53:49 GMT

5f099854-2117-49b3-b252-d6693a85acc5
```

## License

`actix-request-identifier` is licensed under either of the following at your
option:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
