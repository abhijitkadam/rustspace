use tide::Request;

#[derive(Debug, Clone)]
pub struct AppServer{
    info: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {

    let app_server = AppServer{
        info: "service b data".to_string(),
    };    

    let mut app = tide::with_state(app_server);
    app.at("/data").get(data);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

async fn data(req: Request<AppServer>) -> tide::Result {
    Ok(format!("Data: {}", req.state().info).into())
}