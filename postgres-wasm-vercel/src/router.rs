//THIS FILE IS AUTOGENERATED, DO NOT EDIT
use anyhow::Result;
use qaf_router::{WasmRequest, WasmResponse, WasmRouter};

#[path = "pages"]
pub mod pages {
    pub mod ws;
}

pub async fn route(mut req: WasmRequest) -> Result<WasmResponse> {
    let router = WasmRouter::new().get("/", pages::ws::test);

    let matched = router.routes.get(&req.method).unwrap().at(&req.url)?;
    matched.params.iter().for_each(|(k, v)| {
        req.params.insert(k.to_string(), v.to_string());
    });

    let handler = matched.value;
    let resp = handler(req).await;

    return resp;
}
