use axum::debug_handler;
use loco_rs::prelude::*;

#[debug_handler]
pub async fn index(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    format::render().view(&v, "index.html", data!({}))
}

#[debug_handler]
pub async fn blank(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    format::render().view(&v, "blank.html", data!({}))
}

#[debug_handler]
pub async fn calendar(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    format::render().view(&v, "calendar.html", data!({}))
}

#[debug_handler]
pub async fn forms(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    format::render().view(&v, "forms.html", data!({}))
}

#[debug_handler]
pub async fn tables(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    format::render().view(&v, "tables.html", data!({}))
}

#[debug_handler]
pub async fn tabs(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    format::render().view(&v, "tabs.html", data!({}))
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
