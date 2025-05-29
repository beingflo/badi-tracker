use std::time::Duration;

use serde::Serialize;
use tokio::{task, time};
use tungstenite::{Result, connect};

use reqwest::Client;

#[tokio::main]
async fn main() {
    let task = task::spawn(async {
        let mut interval = time::interval(Duration::from_secs(5));

        loop {
            interval.tick().await;
            get_case_count().await.unwrap();
        }
    });

    task.await.unwrap();
}

#[derive(Serialize)]
struct Body {
    bucket: String,
    payload: i32,
}

async fn get_case_count() -> Result<()> {
    let (mut socket, _) = connect("wss://badi-public.crowdmonitor.ch:9591/api").unwrap();
    socket.send("all".into())?;
    let msg = socket.read()?.into_text()?.to_string();

    let mut visitors = 0;
    let value = serde_json::from_str::<serde_json::Value>(&msg).expect("Failed to parse JSON");
    value.as_array().unwrap().iter().for_each(|item| {
        if let Some(uid) = item.get("uid") {
            if uid == "fb012" {
                visitors = item
                    .get("currentfill")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
            }
        }
    });

    let body = Body {
        bucket: "heuried-visitors".to_string(),
        payload: visitors,
    };

    Client::new()
        .post("http://localhost:3000/api/data")
        .body(serde_json::to_string(&body).unwrap())
        .header("Content-Type", "application/json")
        .header(
            "emitter",
            "9Mu6lZEZ87n4DWtflMgiBqGVzaQRRM5CKus366Zo5KLi9qagCST8OlleFSiUgv8K",
        )
        .send()
        .await
        .unwrap();

    socket.close(None)?;

    Ok(())
}
