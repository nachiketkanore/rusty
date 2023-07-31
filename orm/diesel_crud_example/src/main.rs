use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

mod models;
mod schema;

#[macro_use]
extern crate diesel;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(serde::Deserialize)]
struct SampleInput {
    name: String,
    age: i32,
}

#[post("/sample")]
async fn create_sample(pool: web::Data<Pool>, item: web::Json<SampleInput>) -> impl Responder {
    use crate::schema::sample_table::dsl::*;

    let conn = pool.get().expect("Failed to get DB connection from pool");

    let new_sample = models::NewSample {
        name: &item.name,
        age: item.age,
    };

    diesel::insert_into(sample_table)
        .values(&new_sample)
        .execute(&conn)
        .expect("Error inserting new sample");

    HttpResponse::Ok().body("Sample created successfully")
}

#[get("/sample/{sample_id}")]
async fn get_sample(pool: web::Data<Pool>, web::Path(sample_id): web::Path<i32>) -> impl Responder {
    use crate::schema::sample_table::dsl::*;

    let conn = pool.get().expect("Failed to get DB connection from pool");

    let sample = sample_table.find(sample_id).first::<models::Sample>(&conn);

    match sample {
        Ok(sample) => HttpResponse::Ok().json(sample),
        Err(_) => HttpResponse::NotFound().body("Sample not found"),
    }
}

#[put("/sample/{sample_id}")]
async fn update_sample(
    pool: web::Data<Pool>,
    web::Path(sample_id): web::Path<i32>,
    item: web::Json<SampleInput>,
) -> impl Responder {
    use crate::schema::sample_table::dsl::*;

    let conn = pool.get().expect("Failed to get DB connection from pool");

    let updated_sample = models::NewSample {
        name: &item.name,
        age: item.age,
    };

    diesel::update(sample_table.find(sample_id))
        .set(&updated_sample)
        .execute(&conn)
        .expect("Error updating sample");

    HttpResponse::Ok().body("Sample updated successfully")
}

#[delete("/sample/{sample_id}")]
async fn delete_sample(
    pool: web::Data<Pool>,
    web::Path(sample_id): web::Path<i32>,
) -> impl Responder {
    use crate::schema::sample_table::dsl::*;

    let conn = pool.get().expect("Failed to get DB connection from pool");

    diesel::delete(sample_table.find(sample_id))
        .execute(&conn)
        .expect("Error deleting sample");

    HttpResponse::Ok().body("Sample deleted successfully")
}

#[get("/samples")]
async fn get_all_samples(pool: web::Data<Pool>) -> impl Responder {
    use crate::schema::sample_table::dsl::*;

    let conn = pool.get().expect("Failed to get database connection");
    let all_samples_result = sample_table.load::<models::Sample>(&conn);
    match all_samples_result {
        Ok(all_samples) => HttpResponse::Ok().json(all_samples),
        Err(_) => HttpResponse::InternalServerError().body("Failed to get samples"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(create_sample)
            .service(get_sample)
            .service(update_sample)
            .service(delete_sample)
            .service(get_all_samples)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
