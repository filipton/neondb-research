use futures_util::StreamExt;
use qaf_macros::{get, on};
use reqwasm::websocket::{futures::WebSocket, State};
use worker::{console_log, js_sys, EventStream, Request, Response, Result, RouteContext};

#[get]
pub async fn index(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    console_log!("Hello from Rust!");
    let ws = WebSocket::open("wss://ep-steep-mud-147485.eu-central-1.aws.neon.tech/v2").unwrap();
    let mut t = 0;
    while matches!(ws.state(), State::Connecting) {
        gloo_timers::future::sleep(std::time::Duration::from_millis(10)).await;
        t += 10;
    }

    Response::ok(format!("WS: {:?}, T: {}", ws.state(), t))
}
