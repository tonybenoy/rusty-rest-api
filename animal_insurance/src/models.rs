use crate::schema::user_profile;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub Aadhar: String,
    pub age: i32,
    pub income: i32,
    pub risk_questions: Vec<bool>,
    pub recommendation: Option<String>,
    pub Breed: Option<String>,
    pub Number: Option<i32>,
    pub dependents: i32,
}
#[allow(non_snake_case)]
#[derive(Insertable, Debug)]
#[table_name = "user_profile"]
pub struct NewUser {
    pub name: String,
    pub Aadhar: String,
    pub age: i32,
    pub income: i32,
    pub risk_questions: Vec<bool>,
    pub dependents: i32,
    pub Breed: String,
    pub Number: i32,
    pub recommendation: String,
}

impl User {
    pub fn get_cibil_score(&self) -> i32 {
        let score = rand::thread_rng().gen_range(1..999);
        score
    }
}
