use serde::{Deserialize, Serialize};
use tracing::{info, debug, log::warn};
use tracing_subscriber::{Layer, prelude::__tracing_subscriber_SubscriberExt};
use warp::{Filter, hyper::StatusCode, Rejection, reject, Reply};
use std::{io, net::SocketAddr, env, convert::Infallible};


#[derive(Debug)]
struct SecretError;
impl reject::Reject for SecretError {}

#[derive(Deserialize, Serialize, Debug)]
struct Params {
    key: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
struct Response {
    ip: Option<String>,
    err: Option<String>
}


#[tokio::main]
async fn main() {
    let stdout_log = tracing_subscriber::fmt::layer().pretty().with_writer(io::stdout).with_filter(tracing_subscriber::filter::LevelFilter::DEBUG);
    let stderr_log = tracing_subscriber::fmt::layer().pretty().with_writer(io::stderr).with_filter(tracing_subscriber::filter::LevelFilter::ERROR);

    let logger = tracing_subscriber::Registry::default()
        .with(stdout_log)
        .with(stderr_log);

    tracing::subscriber::set_global_default(logger).unwrap();

    let secret = env::var("IP_RESPONSE_SECRET").unwrap_or("1234".to_owned());
    info!("Secret is {}", secret);
    let route = 
        warp::get()
        .and(check_secret(secret))
        .and(warp::addr::remote())
        .and_then(handler)
        .recover(handle_incorrect_secret);

    warp::serve(route).run(([127,0,0,1], 3000)).await;
}


fn check_secret(secret: String) -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::query::<Params>()
    .and(warp::any().map(move || secret.clone()))
    .and_then(check_secret_fn).untuple_one()
}

async fn check_secret_fn(param: Params, secret: String) -> Result<(), Rejection> {
    debug!("Params: {:?}, Secret: {}", param, secret);
    if let Some(param_secret) = param.key {
        if secret.eq(&param_secret) {
            debug!("Matched");
            Ok(())
        } else {
            debug!("Not Matched");
            Err(reject::custom(SecretError))
        }
    } else {
        warn!("Key Not Included");
        Err(reject::custom(SecretError))
    }
}

async fn handler(addr: Option<SocketAddr>) -> Result<impl warp::Reply, Infallible> {
    info!("Handler Hit");
    let reply = match addr {
        None => Response{ip:None, err:Some(String::from("Unable to return ip address"))},
        Some(address) => Response{ip:Some(address.ip().to_string()), err:None},
    };
    info!("Returning {:?}", reply);
    Ok(warp::reply::json(&reply))
}

async fn handle_incorrect_secret(reject: Rejection) -> Result<impl Reply, Rejection> {
    if reject.find::<SecretError>().is_some() {
        Ok(StatusCode::NOT_ACCEPTABLE)
    } else {
        Err(reject)
    }
}

