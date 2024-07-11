use askama::Template;
use axum::{debug_handler, response::Html};
use axum::http::header::SET_COOKIE;
use loco_rs::prelude::*;

use crate::{
    controllers::auth,
    models::users::LoginParams,
};

#[derive(Template)]
#[template(path = "components/login_ok.html")]
pub struct OkTemplate {}

#[debug_handler]
async fn login(state: State<AppContext>, json: Json<LoginParams>) -> Result<Response> {
    let Ok(response) = auth::login(state, json).await else {
        return format::json(());
    };
    let body = response.into_body();
    println!("{:#?}", body);

    let template = OkTemplate {};
    let rendered = template.render().unwrap();
    let mut resp = Html(rendered).into_response();
    resp.headers().insert(SET_COOKIE, "token ".to_string().parse().unwrap());
    Ok(resp)
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/login", post(login))
}