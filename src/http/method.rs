extern crate httparse;
extern crate url;

use std::str;

use super::headers::Headers;

pub trait MethodHandler {
    /// Should this method only perform a TCP connection and not send any bytes?
    fn connect_only(&self) -> bool;

    /// Formulate a response to the original client. An empty Vec will not send
    /// any response directly (responses from a remote will still be proxied).
    fn create_response(&self) -> Vec<u8>;
}

struct ConnectHandler {}

struct DefaultHandler {}

pub fn dispatch<T: MethodHandler>(method: &str) -> T {
    match method.as_ref() {
        "CONNECT" => ConnectHandler::new(),
        _ => DefaultHandler::new(),
    }
}

impl ConnectHandler {
    pub fn new() -> ConnectHandler {
        ConnectHandler{}
    }
}

impl DefaultHandler {
    pub fn new() -> DefaultHandler {
        DefaultHandler{}
    }
}

impl MethodHandler for ConnectHandler {
    fn connect_only(&self) -> bool {
        true
    }

    fn create_response(&self) -> Vec<u8> {
        // TODO: 200 Connection established
        Vec::<u8>::new()
    }
}

impl MethodHandler for DefaultHandler {
    fn connect_only(&self) -> bool {
        false
    }

    fn create_response(&self) -> Vec<u8> {
        Vec::<u8>::new()
    }
}
