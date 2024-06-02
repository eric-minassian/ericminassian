use crate::Response;

const RESUME: &[u8] = include_bytes!("../../assets/Eric_Minassian_resume.pdf");

pub const fn resume() -> Response<'static> {
    Response::new("HTTP/1.1 200 OK", "application/pdf", RESUME)
}
