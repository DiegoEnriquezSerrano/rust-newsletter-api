use crate::models::{NewsletterIssue, PublicNewsletter};
use crate::utils::e404;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, get, web};
use anyhow::Context;
use sqlx::PgPool;

#[get("/newsletters/by_user/{username}/issue/{slug}")]
#[tracing::instrument(name = "Retrieving published newsletter", skip_all)]
pub async fn get(
    pool: web::Data<PgPool>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, actix_web::Error> {
    let params = path.into_inner();
    let newsletter_issue: PublicNewsletter =
        NewsletterIssue::find_public_newsletter(params.0, params.1, &pool)
            .await
            .context("Failed to find newsletter issue.")
            .map_err(e404)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(newsletter_issue))
}
