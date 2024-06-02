use crate::Response;

const INDEX: &[u8] = include_bytes!("../../templates/index.html");

pub const fn index() -> Response<'static> {
    Response::new("HTTP/1.1 200 OK", "text/html", INDEX)
}
