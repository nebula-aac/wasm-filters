use log::trace;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::time::Duration;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> { Box::new(UpstreamCall::new()) });
}}

#[derive(Debug)]
struct UpstreamCall {}

impl UpstreamCall {
    fn new() -> Self {
        return Self {};
    }
}

impl HttpContext for UpstreamCall {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        let token = self
            .get_http_request_header("token")
            .unwrap_or(String::from(""));
        proxy_wasm::hostcalls::log(LogLevel::Info, format!("Auth header: {:?}", token).as_str())
            .ok();
        let x = self.dispatch_http_call(
            "wasm_upstream",
            vec![
                (":method", "GET"),
                (":path", "/auth"),
                (":authority", "wasm_upstream"),
                ("token", token.as_str()),
            ],
            None,
            vec![],
            Duration::from_secs(5),
        );
        proxy_wasm::hostcalls::log(LogLevel::Trace, format!("{:?}", x).as_str()).ok();
        Action::Continue
    }
    fn on_http_response_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        Action::Continue
    }
}

impl Context for UpstreamCall {
    fn on_http_call_response(
        &mut self,
        _token_id: u32,
        _num_headers: usize,
        _body_size: usize,
        _num_trailers: usize,
    ) {
        if let Some(body) = self.get_http_call_response_body(0, _body_size) {
            if let Ok(body) = std::str::from_utf8(&body) {
                proxy_wasm::hostcalls::log(
                    LogLevel::Info,
                    format!("HTTP Call body : {:?} {:?}", body, body == "Authorized").as_str(),
                )
                .ok();
                if body == "Authorized" {
                    self.resume_http_request();
                    return;
                }

                trace!("Access forbidden.");
                self.send_http_response(
                    403,
                    vec![("Powered-By", "proxy-wasm")],
                    Some(b"Access forbidden.\n"),
                );
            }
        }
    }
}
impl RootContext for UpstreamCall {}
