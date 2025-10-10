use axum::Json;

pub async fn ping() -> Json<String>{
    return Json::from(String::from("Pong!"));
}
