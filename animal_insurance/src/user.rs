use crate::schema::user_profile;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub Aadhar: String,
    pub age: i32,
    pub recomendation: String,
    pub income: i32,
    pub risk_questions: Vec<bool>,
    Breed: String,
    Number: i32,
}
#[allow(non_snake_case)]
#[derive(Insertable, Debug)]
#[table_name = "user_profile"]
pub struct NewUser {
    name: String,
    Aadhar: String,
    age: i32,
    income: i32,
    risk_questions: Vec<bool>,
    Breed: String,
    Number: i32,
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^\d+$").unwrap();
}
#[allow(non_snake_case)]
#[derive(Deserialize, Validate)]
pub struct UserInfo {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(equal = 12))]
    pub Aadhar: String,
    #[validate(range(min = 1))]
    pub age: i32,
    #[validate(range(min = 1))]
    pub income: i32,
    pub risk_questions: Vec<bool>,
    pub cattle: CattleInfo,
}
#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct CattleInfo {
    pub Breed: String,
    pub Number: i32,
}
