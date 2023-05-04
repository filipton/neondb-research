use qaf_macros::{get, on};
use worker::{
    console_log, EventStream, Request, Response, Result, RouteContext, WebSocket, WebSocketPair,
    WebsocketEvent,
};

#[get]
pub async fn index(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let ws = WebSocket::connect("ws://127.0.0.1:8080".parse()?).await?;

    // It's important that we call this before we send our first message, otherwise we will
    // not have any event listeners on the socket to receive the echoed message.
    let mut event_stream = ws.events()?;

    ws.accept()?;
    ws.send_with_str("Hello, world!")?;

    /*
    while let Some(event) = event_stream.next().await {
        let event = event?;

        if let WebsocketEvent::Message(msg) = event {
            if let Some(text) = msg.text() {
                return Response::ok(text);
            }
        }
    }
    */

    Response::error("never got a message echoed back :(", 500)
}
