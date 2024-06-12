use loco_rs::prelude::*;
use askama::Template;
use axum::response::Html;
use axum::debug_handler;

#[derive(Template)]
#[template(path="blank.html")]
pub struct BlankTemplate {}

#[derive(Template)]
#[template(path="calendar.html")]
pub struct CalnedarTemplate {}

#[derive(Template)]
#[template(path="forms.html")]
pub struct FormsTemplate {}

#[derive(Template)]
#[template(path="index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path="tables.html")]
pub struct TablesTemplate {}

#[derive(Template)]
#[template(path="tabs.html")]
pub struct TabsTemplate {}

/// # Panics
/// 
/// Will panic if unwrap panics
#[debug_handler]
pub async fn blank() -> Result<Response> {
    let template = BlankTemplate {};
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}

/// # Panics
/// 
/// Will panic if unwrap panics
#[debug_handler]
pub async fn calendar() -> Result<Response> {
    let template = CalnedarTemplate {};
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}

/// # Panics
/// 
/// Will panic if unwrap panics
#[debug_handler]
pub async fn forms() -> Result<Response> {
    let template = FormsTemplate {};
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}

/// # Panics
/// 
/// Will panic if unwrap panics
#[debug_handler]
pub async fn index() -> Result<Response> {
    let template = IndexTemplate {};
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}

/// # Panics
/// 
/// Will panic if unwrap panics
#[debug_handler]
pub async fn tables() -> Result<Response> {
    let template = TablesTemplate {};
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}

/// # Panics
/// 
/// Will panic if unwrap panics
#[debug_handler]
pub async fn tabs() -> Result<Response> {
    let template = TabsTemplate {};
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/")
        .add("/", get(index))
        .add("/index", get(index))
        .add("/forms", get(forms))
        .add("/tables", get(tables))
        .add("/blank", get(blank))
        .add("/calendar", get(calendar))
        .add("/tabs", get(tabs))
}