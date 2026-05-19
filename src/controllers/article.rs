#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use axum::extract::Query;
use loco_rs::model::query::{PageResponse, PaginationQuery};
use sea_orm::Condition;

use crate::models::_entities::articles::{ActiveModel, Column, Entity, Model};
use chrono::NaiveDate;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub title: Option<String>,
    pub content: Option<String>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.title = Set(self.title.clone());
        item.content = Set(self.content.clone());
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams {
    pub title: Option<String>,
    pub content: Option<String>,
    pub created_at_from: Option<String>,
    pub created_at_to: Option<String>,
    #[serde(flatten)]
    pub pagination_query: PaginationQuery,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListResponse {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl From<Model> for ListResponse {
    fn from(article: Model) -> Self {
        Self {
            id: article.id,
            title: article.title.clone(),
            content: article.content.clone(),
            created_at: article.created_at,
            updated_at: article.updated_at,
        }
    }
}

pub async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn list_inner(
    ctx: &AppContext,
    query_params: &QueryParams,
) -> Result<PageResponse<Model>> {
    let mut condition = Condition::all();
    if let Some(ref title) = query_params.title {
        if !title.is_empty() {
            condition = condition.add(Column::Title.contains(title));
        }
    }
    if let Some(ref content) = query_params.content {
        if !content.is_empty() {
            condition = condition.add(Column::Content.contains(content));
        }
    }
    if let Some(ref from) = query_params.created_at_from {
        if !from.is_empty() {
            if let Ok(dt) = NaiveDate::parse_from_str(from, "%Y-%m-%dT%H:%M") {
                condition = condition.add(Column::CreatedAt.gte(dt));
            }
        }
    }
    if let Some(ref to) = query_params.created_at_to {
        if !to.is_empty() {
            if let Ok(dt) = NaiveDate::parse_from_str(to, "%Y-%m-%dT%H:%M") {
                condition = condition.add(Column::CreatedAt.lte(dt));
            }
        }
    }

    model::query::paginate(
        &ctx.db,
        Entity::find(),
        Some(condition),
        &query_params.pagination_query,
    )
    .await
}

#[debug_handler]
pub async fn list(
    Query(query_params): Query<QueryParams>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let response = list_inner(&ctx, &query_params).await?;
    let items: Vec<ListResponse> = response.page.into_iter().map(ListResponse::from).collect();
    format::json(data!({
        "results": items,
        "pagination": {
            "page": query_params.pagination_query.page,
            "page_size": query_params.pagination_query.page_size,
            "total_pages": response.total_pages,
        }
    }))
}

pub async fn add_inner(ctx: &AppContext, params: Params) -> Result<Model> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    Ok(item)
}

pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let item = add_inner(&ctx, params).await?;
    format::json(item)
}

pub async fn update_inner(id: i32, ctx: &AppContext, params: Params) -> Result<Model> {
    let item: Model = load_item(ctx, id).await?;
    let mut item: ActiveModel = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    Ok(item)
}

pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let item = update_inner(id, &ctx, params).await?;
    format::json(item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/articles")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", get(get_one))
        .add("/{id}", delete(remove))
        .add("/{id}", post(update))
}
