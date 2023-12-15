use anyhow::anyhow;
use spin_sdk::http::{IntoResponse, Method, Request};
use spin_sdk::http_component;
use spin_sdk::key_value::Store;

/// A simple Spin HTTP component.
#[http_component]
fn handle_data(req: Request) -> anyhow::Result<impl IntoResponse> {
    let kv_store = Store::open_default()?;
    match *req.method() {
        Method::Get => {
            let body = kv_store.get(req.query())?.unwrap_or(vec![]);
            Ok(http::Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(body)?)
        }
        Method::Post => {
            kv_store.set(req.query(), req.body())?;
            Ok(http::Response::builder().status(201).body(vec![])?)
        }
        _ => Err(anyhow!("expected get or post")),
    }
}
