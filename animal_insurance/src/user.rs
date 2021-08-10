use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use validator::Validate;

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
    #[validate(range(min = 0))]
    pub age: i32,
    #[validate(range(min = 0))]
    pub income: i32,
    pub risk_questions: Vec<i32>,
    pub cattle: Option<CattleInfo>,
    #[validate(range(min = 0))]
    pub dependents: i32,
}
#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct CattleInfo {
    pub Breed: String,
    pub Number: i32,
}
impl UserInfo {
    pub fn get_breed_if_available(&self) -> String {
        match &self.cattle {
            Some(cattle) => return cattle.Breed.clone(),
            None => return "".to_string(),
        }
    }
    pub fn get_num_if_available(&self) -> i32 {
        match &self.cattle {
            Some(cattle) => return cattle.Number,
            None => return 0,
        }
    }
}
