use crate::languages::managers::LanguageManager;

use actix_web::{get, HttpResponse, web::{Data}};
use sqlx::postgres::PgPool;
use tracing::{instrument};


#[instrument(name="Getting all languages", skip(db_pool))]
#[get("/all")]
pub async fn get_all_languages(db_pool: Data<PgPool>) -> HttpResponse {
    let manager = LanguageManager {
        db_pool: db_pool.clone(),
    };
    match manager.get_languages().await {
        Ok(languages) => return HttpResponse::Ok().json(languages),
        Err(e) => return HttpResponse::InternalServerError().json(e)
    };
}