//!
use actix_web::{web, HttpResponse};
use sqlx::{query, PgConnection};
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct FormData {
    //email: String,
    //name: String
}

pub async fn subscribe(
    form: web::Form<FormData>,
    connection: web::Data<PgConnection>,    
) -> HttpResponse {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, sub_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    ).execute(connection.get_ref()).await;
    
    HttpResponse::Ok().finish()
}
