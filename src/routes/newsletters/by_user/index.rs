use crate::models::{NewsletterIssue, PublicNewsletterListItem};
use crate::utils::e500;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, get, web};
use anyhow::Context;
use sqlx::PgPool;

#[get("/newsletters/by_user/{username}")]
#[tracing::instrument(name = "Retrieving list of published newsletters by user", skip_all)]
pub async fn get(
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, actix_web::Error> {
    let params = path.into_inner();
    let username = params.0;
    let newsletter_issues: Vec<PublicNewsletterListItem> =
        NewsletterIssue::get_public_newsletters_by_username(username, &pool)
            .await
            .context("Failed to query newsletter issues.")
            .map_err(e500)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(newsletter_issues))
}
