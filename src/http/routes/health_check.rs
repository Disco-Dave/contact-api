use actix_web::HttpResponse;

#[tracing::instrument(name = "health-check handler")]
pub fn handler() -> HttpResponse {
    tracing::info!("Executing health-check handler");
    HttpResponse::NoContent().finish()
}
