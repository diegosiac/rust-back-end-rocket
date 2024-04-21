#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod schema;

use rocket::serde::json::Json;

use diesel::prelude::*;
use rocket_sync_db_pools::{database, diesel::PgConnection};

use schema::tasks;
use serde::{Deserialize, Serialize};

#[database("postgres_database")]
struct DbConn(PgConnection);

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = tasks)]
struct Task {
    id: i32,
    title: String,
    description: String,
    completed: bool,
}

#[post("/task", format = "json", data = "<task>")]
async fn create_task(task: Json<Task>, db: DbConn) -> Result<Json<Task>, String> {
    db.run(move |c| {
        diesel::insert_into(tasks::table)
            .values(task.into_inner())
            .get_result(c)
            .map_err(|_| "Error adding task".to_string())
            .map(Json)
    })
    .await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![create_task])
}
