use worker::*;
mod router;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    return router::route(req, env).await;
}

