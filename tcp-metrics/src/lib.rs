use std::time::SystemTime;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_stream_context(|_, _| -> Box<dyn StreamContext> { Box::new(TCPMetrics::new()) });
}}

#[derive(Debug)]
struct TCPMetrics {
    data_downstream: usize,
    data_upstream: usize,
    time: SystemTime,
    latency: u128,
}

impl TCPMetrics {
    fn new() -> Self {
        return Self {
            data_downstream: 0,
            data_upstream: 0,
            time: SystemTime::UNIX_EPOCH,
            latency: 0,
        };
    }
}

impl StreamContext for TCPMetrics {
    fn on_downstream_data(&mut self, _data_size: usize, _end_of_stream: bool) -> Action {
        self.data_downstream += _data_size;
        Action::Continue
    }

    fn on_upstream_data(&mut self, _data_size: usize, _end_of_stream: bool) -> Action {
        self.data_upstream += _data_size;
        Action::Continue
    }

    fn on_downstream_close(&mut self, _peer_type: PeerType) {
        if let Ok(curr_time) = proxy_wasm::hostcalls::get_current_time() {
            if let Ok(dur) = curr_time.duration_since(self.time) {
                self.latency = dur.as_micros()
            }
        }
        proxy_wasm::hostcalls::log(LogLevel::Trace, format!("{:?}", self).as_str()).ok();
    }

    fn on_upstream_close(&mut self, _peer_type: PeerType) {
        if let Ok(curr_time) = proxy_wasm::hostcalls::get_current_time() {
            self.time = curr_time;
        }
    }
}
impl Context for TCPMetrics {}
impl RootContext for TCPMetrics {}
