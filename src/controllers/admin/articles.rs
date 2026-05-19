use axum::debug_handler;
use loco_rs::prelude::*;

use axum::extract::Query;

use crate::controllers::article;
use crate::controllers::article::QueryParams;
use std::cmp;

#[debug_handler]
pub async fn render(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    format::render().view(&v, "articles.html", data!({}))
}

fn page_numbers(page: u64, total_pages: u64) -> Vec<u64> {
    let prange: u64 = 5;
    let section = if page > 0 { (page - 1) / prange } else { 0 };
    let start = prange * section + 1;
    let end = cmp::min(prange * (section + 1), total_pages);
    (start..=end).collect()
}

#[debug_handler]
pub async fn list(
    Query(query_params): Query<QueryParams>,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let response = article::list_inner(&ctx, &query_params).await?;
    let page = query_params.pagination_query.page;
    let total_pages = response.total_pages;
    format::render().view(
        &v,
        "articles/list.html",
        data!({
            "rows": response.page,
            "page": page,
            "total_pages": total_pages,
            "page_numbers": page_numbers(page, total_pages),
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
        .add("/:id/edit", get(edit))
        .add("/:id", put(update))
        .add("/", post(add))
}
