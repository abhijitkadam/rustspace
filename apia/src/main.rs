use tide::Request;
use std::env;

#[derive(Debug, Clone)]
pub struct AppServer{
    ext_service: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {       

    let mut app_server = AppServer{
        ext_service: "http://localhost:8080/data".to_string(),
    };
    if let Ok(service) = env::var("apibservice") {
        app_server.ext_service = format!("http://{}/data", service)
    }

    let mut app = tide::with_state(app_server);
    app.at("/info").get(info);
    app.listen("0.0.0.0:8081").await?;
    Ok(())
}

async fn info(req: Request<AppServer>) -> tide::Result {
    let res = surf::get(req.state().ext_service.clone()).await?.body_string().await?;
    Ok(format!("Data from source is : {}", res).into())
}