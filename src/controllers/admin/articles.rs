use loco_rs::prelude::*;
use askama::Template;
use axum::response::Html;
use axum::debug_handler;

use loco_rs::model::query::PageResponse;
use axum::extract::Query;
use query::PaginationQuery;

use crate::controllers::article;
use crate::controllers::article::QueryParams;
use crate::models::_entities::articles::Model;
use std::cmp;

#[derive(Template)]
#[template(path="articles.html")]
pub struct PageTemplate {}

#[debug_handler]
pub async fn render() -> Result<Response> {
    let template = PageTemplate {};
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}

#[derive(Template)]
#[template(path="components/article_form_edit.html")]
pub struct ItemTemplate {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
impl ItemTemplate {
    fn from(item: Model) -> Self {
        Self {
            id: item.id,
            title: item.title.unwrap_or_default(),
            content: item.content.unwrap_or_default(),
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

#[derive(Template)]
#[template(path="components/article_form_new.html")]
pub struct NewTemplate {}

#[derive(Template)]
#[template(path = "components/article_table.html")]
pub struct ListTemplate {
    pub rows: Vec<ItemTemplate>,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
}

impl ListTemplate {
    const fn new() -> Self {
        Self {
            rows: Vec::<ItemTemplate>::new(),
            page: 0,
            page_size: 0,
            total_pages: 0,
        }
    }
    fn add(&mut self, elem: ItemTemplate) {
        self.rows.push(elem);
    }
    
    fn build(response: PageResponse<Model>, pagination_query: &PaginationQuery) -> Self {
        let mut template: ListTemplate = response.page.into_iter().map(ItemTemplate::from).collect();
        template.page = pagination_query.page;
        template.page_size = pagination_query.page_size;
        template.total_pages = response.total_pages;
        template
    }
}

impl FromIterator<ItemTemplate> for ListTemplate {
    fn from_iter<U>(iter: U) -> Self
    where U: IntoIterator<Item=ItemTemplate> {
        let mut c = Self::new();

        for i in iter {
            c.add(i);
        }
        c
    }
}

/// # Panics
/// 
/// Will panic if unwrap panics
/// # Errors
///
/// Will return 'Err' if something goes wrong
#[debug_handler]
pub async fn list(Query(query_params): Query<QueryParams>, State(ctx): State<AppContext>) -> Result<Response> {
    let response = article::list_inner(&ctx, &query_params).await?;
    let template = ListTemplate::build(response, &query_params.pagination_query);
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}

/// # Panics
/// 
/// Will panic if unwrap panics
/// # Errors
/// 
/// Will return 'Err' if something goes wrong
#[debug_handler]
pub async fn new() -> Result<Response> {
    let template = NewTemplate {};
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}

/// # Panics
/// 
/// Will panic if unwrap panics
/// # Errors
/// 
/// Will return 'Err' if something goes wrong
#[debug_handler]
pub async fn edit(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let item = article::load_item(&ctx, id).await?;
    let template = ItemTemplate::from(item);
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}

/// # Panics
/// 
/// Will panic if unwrap panics
/// # Errors
/// 
/// Will return 'Err' if something goes wrong
#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<article::Params>) -> Result<Response> {
    let item = article::add_inner(&ctx, params).await?;
    let template = ItemTemplate::from(item);
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
}

/// # Panics
/// 
/// Will panic if unwrap panics
/// # Errors
/// 
/// Will return 'Err' if something goes wrong
#[debug_handler]
pub async fn update(Path(id): Path<i32>, State(ctx): State<AppContext>, Json(params): Json<article::Params>) -> Result<Response> {
    let item = article::update_inner(id, &ctx, params).await?;
    let template = ItemTemplate::from(item);
    let rendered = template.render().unwrap();
    Ok(Html(rendered).into_response())
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