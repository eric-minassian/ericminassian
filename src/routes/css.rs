use crate::Response;

const CSS: &[u8] = include_bytes!("../../build/index.css");

pub const fn css() -> Response<'static> {
    Response::new("HTTP/1.1 200 OK", "text/css", CSS)
}
