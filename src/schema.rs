// @generated automatically by Diesel CLI.

diesel::table! {
    attribute (id) {
        id -> Integer,
        bird_id -> Integer,
        attributor_id -> Integer,
        bio -> Longtext,
        #[max_length = 255]
        link -> Varchar,
    }
}

diesel::table! {
    attributor (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        bio -> Longtext,
    }
}

diesel::table! {
    bird (id) {
        id -> Integer,
        #[max_length = 255]
        common_name -> Varchar,
        #[max_length = 255]
        commonwealth_status -> Varchar,
        #[max_length = 255]
        nsw_status -> Varchar,
        #[max_length = 255]
        profile -> Varchar,
        #[max_length = 255]
        scientific_name -> Varchar,
        #[max_length = 255]
        stats -> Varchar,
        #[max_length = 255]
        stats_for -> Varchar,
    }
}

diesel::table! {
    bird_threat (bird_id, threat_id) {
        bird_id -> Integer,
        threat_id -> Integer,
    }
}

diesel::table! {
    threat (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(attribute -> attributor (attributor_id));
diesel::joinable!(attribute -> bird (bird_id));
diesel::joinable!(bird_threat -> bird (bird_id));
diesel::joinable!(bird_threat -> threat (threat_id));

diesel::allow_tables_to_appear_in_same_query!(
    attribute,
    attributor,
    bird,
    bird_threat,
    threat,
);
