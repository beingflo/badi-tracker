use std::{env, time::Duration};

use serde::Serialize;
use tokio::{task, time};
use tungstenite::{Result, connect};

use reqwest::Client;

#[tokio::main]
async fn main() {
    println!("Starting badi-tracker");

    let (_, emitter) = env::vars().find(|v| v.0.eq("EMITTER")).unwrap();
    println!("Loaded emmiter from .env file");

    let task = task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(5));

        loop {
            interval.tick().await;
            record_visitors(emitter.clone()).await.unwrap();
        }
    });

    println!("Starting task");
    task.await.unwrap();
}

#[derive(Serialize)]
struct Body {
    bucket: String,
    payload: i32,
}

async fn record_visitors(emitter: String) -> Result<()> {
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
        .post("https://observatory.marending.dev/api/data")
        .body(serde_json::to_string(&body).unwrap())
        .header("Content-Type", "application/json")
        .header("emitter", emitter)
        .send()
        .await
        .unwrap();

    println!("Recorded {} visitors", visitors);

    socket.close(None)?;

    Ok(())
}
