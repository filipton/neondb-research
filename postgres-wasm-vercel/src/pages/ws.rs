use anyhow::Result;
use qaf_macros::{get, post};
use qaf_router::{WasmRequest, WasmResponse};
use reqwasm::websocket::{futures::WebSocket, State};

#[get("")]
pub async fn test(req: WasmRequest) -> Result<WasmResponse> {
    let ws = WebSocket::open("wss://ep-steep-mud-147485.eu-central-1.aws.neon.tech/v2").unwrap();
    let mut t = 0;
    while matches!(ws.state(), State::Connecting) {
        gloo_timers::future::sleep(std::time::Duration::from_millis(10)).await;
        t += 10;
    }

    Ok(WasmResponse::ok(&format!("WS: {:?}, T: {}", ws.state(), t)))
}
