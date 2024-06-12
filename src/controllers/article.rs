#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use axum::debug_handler;

use axum::extract::Query;
use sea_orm::Condition;
use loco_rs::model::query::{PaginationQuery, PageResponse};
use loco_rs::controller::views::pagination::{Pager, PagerMeta};

use crate::models::_entities::articles::{ActiveModel, Entity, Model, Column};

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
    #[serde(flatten)]                                       // added
    pub pagination_query: PaginationQuery,                  // added
}

#[derive(Debug, Deserialize,Serialize)]
pub struct ListResponse {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct PaginationResponse {}

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
impl PaginationResponse {
    #[must_use]
    pub fn response(
        data: PageResponse<Model>,
        pagination_query: &PaginationQuery,
    ) -> Pager<Vec<ListResponse>> {
        Pager {
            results: data
            .page
            .into_iter()
            .map(ListResponse::from)
            .collect::<Vec<ListResponse>>(),
            info: PagerMeta {
                page: pagination_query.page,
                page_size: pagination_query.page_size,
                total_pages: data.total_pages,
            },
        }
    }
}

pub async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn list_inner(ctx: &AppContext, query_params: &QueryParams) -> Result<PageResponse<Model>> {
    let title_filter = query_params.title.as_ref().unwrap_or(&String::new()).clone();
    let content_filter = query_params.content.as_ref().unwrap_or(&String::new()).clone();
    let mut condition = Condition::all();
    if !title_filter.is_empty() {
        condition = condition.add(Column::Title.contains(&title_filter));
    }
    if !content_filter.is_empty() {
        condition = condition.add(Column::Content.contains(&content_filter));
    }
    model::query::paginate(
        &ctx.db, Entity::find(), Some(condition), &query_params.pagination_query
    ).await
}

#[debug_handler]
pub async fn list(Query(query_params): Query<QueryParams>, State(ctx): State<AppContext>) -> Result<Response> {
    let response = list_inner(&ctx, &query_params).await?;
    format::json(PaginationResponse::response(response, &query_params.pagination_query))
}

pub async fn add_inner(ctx: &AppContext, params: Params) -> Result<Model> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await;
    match item {
        Ok(v) => Ok(v),
        Err(err) => core::result::Result::Err(loco_rs::Error::DB(err)),
    }
}

pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let item = add_inner(&ctx, params).await?;
    format::json(item)
}

pub async fn update_inner(id: i32, ctx: &AppContext, params: Params) -> Result<Model> {
    let item: Model = load_item(&ctx, id).await?;
    let mut item: ActiveModel = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await;
    match item {
        Ok(v) => Ok(v),
        Err(err) => core::result::Result::Err(loco_rs::Error::DB(err)),
    }
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
        .add("/:id", get(get_one))
        .add("/:id", delete(remove))
        .add("/:id", post(update))
}
