-- Your SQL goes here
CREATE TABLE public.user_profile (
    id serial NOT NULL,
    "name" varchar NOT NULL,
    "Aadhar" varchar NOT NULL,
    age integer NOT NULL,
    income integer NOT NULL,
    risk_questions boolean ARRAY NOT NULL,
    recomendation varchar NULL,
    "Breed" varchar NOT NULL,
    "Number" integer NOT NULL,
    dependents integer NOT NULL,
    CONSTRAINT user_profile_pk PRIMARY KEY (id)
);



CREATE UNIQUE INDEX user_profile_aadhaar_idx ON public.user_profile ("Aadhar");
