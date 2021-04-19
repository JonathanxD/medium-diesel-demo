#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use schema::users;
use schema::users::table as users_table;
use schema::users::id as users_id;

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let connection = establish_connection();
    let new_user = NewUser {
        name: "medium",
        email: "medium@example.com"
    };

    let user: User = diesel::insert_into(users_table)
        .values(&new_user)
        .get_result(&connection)
        .expect("Error saving new user");

    println!("Saved user {:?}", user);

    let users_result = users_table.filter(users_id.eq(1))
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    for user in users_result {
        println!("Found user: {:?}", user);
    }
}