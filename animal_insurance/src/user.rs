use crate::schema::user_profile;
use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub Aadhaar: String,
    pub age: i32,
    pub recomendation: String,
    pub income: i32,
    pub risk_question: Vec<bool>,
    Breed: String,
    Number: i32,
}
#[allow(non_snake_case)]
#[derive(Insertable, Debug)]
#[table_name = "user_profile"]
pub struct NewUser {
    name: String,
    Aadhaar: String,
    age: i32,
    income: i32,
    risk_question: Vec<bool>,
    Breed: String,
    Number: i32,
}
