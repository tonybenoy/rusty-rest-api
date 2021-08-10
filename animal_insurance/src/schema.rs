table! {
    user_profile (id) {
        id -> Int4,
        name -> Varchar,
        Aadhar -> Varchar,
        age -> Int4,
        income -> Int4,
        risk_questions -> Array<Bool>,
        recommendation -> Nullable<Varchar>,
        Breed -> Nullable<Varchar>,
        Number -> Nullable<Int4>,
        dependents -> Int4,
    }
}
