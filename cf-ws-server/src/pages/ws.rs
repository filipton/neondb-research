use futures::StreamExt;
use qaf_macros::{get, on};
use worker::{
    console_log, EventStream, Request, Response, Result, RouteContext, WebSocket, WebSocketPair,
    WebsocketEvent,
};

#[get]
pub async fn index(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("Hello, World!")
}

#[on("ws")]
pub async fn ws(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let upgrade_header = req.headers().get("Upgrade");
    if !upgrade_header.is_ok() || upgrade_header.unwrap() != Some("websocket".to_string()) {
        return Ok(Response::ok("Websocket Error").unwrap().with_status(426));
    }

    let web_socket_pair = WebSocketPair::new()?;
    let client = web_socket_pair.client;
    let server = web_socket_pair.server;
    websocket(server).await?;

    return Ok(Response::from_websocket(client).unwrap().with_status(101));
}

async fn websocket(ws: WebSocket) -> Result<()> {
    ws.accept()?;
    ws.send(&"Connected!".to_string())?;

    wasm_bindgen_futures::spawn_local(async move {
        let mut event_stream: EventStream = ws.events().expect("Failed to get event stream");
        while let Some(event) = event_stream.next().await {
            match event.expect("Failed to get event") {
                WebsocketEvent::Message(msg) => {
                    if let Some(text) = msg.text() {
                        ws.send_with_str(text).expect("Failed to send message");
                    }
                }
                _ => {}
            }
        }
    });
    Ok(())
}
