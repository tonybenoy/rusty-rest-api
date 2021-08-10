table! {
    user_profile (id) {
        id -> Int4,
        name -> Varchar,
        Aadhaar -> Varchar,
        age -> Int4,
        income -> Int4,
        risk_question -> Array<Bool>,
        recomendation -> Nullable<Varchar>,
        Breed -> Varchar,
        Number -> Int4,
    }
}
