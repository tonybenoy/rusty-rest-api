table! {
    user_profile (id) {
        id -> Int4,
        name -> Varchar,
        Aadhar -> Varchar,
        age -> Int4,
        income -> Int4,
        risk_questions -> Array<Bool>,
        recomendation -> Nullable<Varchar>,
        Breed -> Varchar,
        Number -> Int4,
    }
}
