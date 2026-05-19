use axum::debug_handler;
use loco_rs::prelude::*;

use axum::extract::Query;

use crate::controllers::article;
use crate::controllers::article::QueryParams;

#[debug_handler]
pub async fn render(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    format::render().view(&v, "articles.html", data!({}))
}

#[debug_handler]
pub async fn list(
    Query(query_params): Query<QueryParams>,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let response = article::list_inner(&ctx, &query_params).await?;
    format::render().view(
        &v,
        "articles/list.html",
        data!({
            "rows": response.page,
            "page": query_params.pagination_query.page,
            "page_size": query_params.pagination_query.page_size,
            "total_pages": response.total_pages,
        }),
    )
}

#[debug_handler]
pub async fn new(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    format::render().view(&v, "articles/form_new.html", data!({}))
}

#[debug_handler]
pub async fn edit(
    Path(id): Path<i32>,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = article::load_item(&ctx, id).await?;
    format::render().view(&v, "articles/form_edit.html", data!({"item": item}))
}

#[debug_handler]
pub async fn add(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
    Json(params): Json<article::Params>,
) -> Result<Response> {
    let item = article::add_inner(&ctx, params).await?;
    format::render().view(&v, "articles/form_edit.html", data!({"item": item}))
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
    Json(params): Json<article::Params>,
) -> Result<Response> {
    let item = article::update_inner(id, &ctx, params).await?;
    format::render().view(&v, "articles/form_edit.html", data!({"item": item}))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/articles")
        .add("/", get(render))
        .add("/list", get(list))
        .add("/new", get(new))
        .add("/{id}/edit", get(edit))
        .add("/{id}", put(update))
        .add("/", post(add))
}
