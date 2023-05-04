//THIS FILE IS AUTOGENERATED, DO NOT EDIT
use worker::{Context, Env, Request, Response, Result, Router};

#[path = "pages"]
pub mod pages {
    pub mod tests;
}

pub async fn route(req: Request, env: Env) -> Result<Response> {
    let router = Router::new();

    return router
        .get_async("/", pages::tests::index)
        .run(req, env)
        .await;
}
