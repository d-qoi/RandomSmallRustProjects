use std::{error::Error, io::Read};

use bytes::{Buf, Bytes};
use futures::{StreamExt, TryStreamExt};
use tracing::{error, info};
use warp::{
    hyper::{HeaderMap, Uri},
    multipart::{FormData, Part},
    Filter, Rejection, Reply,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let route = warp::post()
        .and(warp::path("submit"))
        .and(warp::path::end())
        .and(warp::multipart::form().max_length(4 * 1024 * 1024)) // set multipart form.
        .and(warp::header::headers_cloned())
        .and_then(handler);

    let files = warp::get().and(warp::fs::file("src/index.html"));

    let env = std::env::current_dir();
    info!("{:?}", env);

    warp::serve(files.or(route))
        .run(([127, 0, 0, 1], 8081))
        .await;
    Ok(())
}

async fn handler(form: FormData, headers: HeaderMap) -> Result<impl Reply, Rejection> {
    info!("{:?}", headers);
    info!("Form Data: {:?}", form);
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        error!("{:?}", e);
        warp::reject::reject()
    })?;
    info!("parts: {:?}", parts);
    for mut part in parts {
        info!("------------------------------------");
        if let Some(Ok(data)) = part.data().await {
            let mut dest = vec![];
            let mut reader = data.reader();
            let res = std::io::copy(&mut reader, &mut dest).unwrap();
            info!("Size: {}", res);
        } else {
            error!("Error while getting data.");
        }
        // let mut b = part.stream().try_collect::<Bytes>().await.map_err(|e| {
        //     error!("Unknown Error: {:?}", e);
        //     warp::reject::reject()
        // })?;
        // let bytes = b.iter_mut().map(|chunk| chunk.get_u8()).collect::<Vec<_>>();
        // info!("{:?}", bytes);
    }

    Ok(warp::redirect(Uri::from_static("/")))
}
