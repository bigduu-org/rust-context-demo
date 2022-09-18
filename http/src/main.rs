use std::net::SocketAddr;
use std::sync::Arc;
use std::time;

use axum::{Json, Router};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use lazy_static::lazy_static;
use log::info;
use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use tokio::sync::RwLock;

use core::configuration::builder;
use core::context::DefaultApplicationContext;

async fn mongo() -> Result<(), mongodb::error::Error> {
    let result = ClientOptions::parse_connection_string("mongodb://localhost:27017".parse().unwrap()).await?;
    let client = mongodb::Client::with_options(result)?;
    let collection = client.database("rust").collection("test");
    collection.drop(None).await?;
    let instant = time::Instant::now();
    let mut join_handler_vec = vec![];
    for i in 0..99999 {
        let collection1 = collection.clone();
        let handle = tokio::spawn(async move {
            collection1.insert_one(doc! {"456":i}, None).await.unwrap();
            info!("insert {} finished",i);
        });
        join_handler_vec.push(handle);
    }
    futures::future::join_all(join_handler_vec).await;
    info!("Insert 1000000 documents use {} sec",instant.elapsed().as_secs_f64());
    Ok(())
}

async fn main_async(application_context: Box<DefaultApplicationContext>) {
    CONTEXT.write().await.context = *application_context;
    info!("this is from async");
    // mongo().await.unwrap();
    let router = Router::new()
        .route("/", get(root))
        .route("/configuration", get(get_configuration));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn get_configuration() -> impl IntoResponse {
    let configuration = CONTEXT.read().await.context.get_configuration();
    let properties = configuration.properties;
    (StatusCode::OK, Json(properties))
}

async fn root() -> &'static str {
    "hello,world"
}

#[derive(Debug, Default)]
struct ContextHolder {
    context: DefaultApplicationContext,
}

type SharedContext = Box<Arc<RwLock<ContextHolder>>>;

lazy_static! {
    static ref CONTEXT: SharedContext = Default::default();
}

fn main() {
    let (context, runtime) = builder::ApplicationBuilder::builder().resource_builder().build();
    runtime.block_on(main_async(context))
}
