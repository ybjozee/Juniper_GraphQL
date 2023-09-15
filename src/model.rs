use diesel::prelude::*;
use juniper::{GraphQLInputObject, GraphQLObject};

use crate::schema::*;

#[derive(GraphQLObject, Queryable, Insertable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Bird))]
#[diesel(belongs_to(Attributor))]
#[diesel(table_name = attribute)]
pub struct Attribute {
    pub id: i32,
    pub bird_id: i32,
    pub attributor_id: i32,
    pub bio: String,
    pub link: String,
}

#[derive(GraphQLObject, Queryable, Identifiable)]
#[diesel(table_name = attributor)]
pub struct Attributor {
    pub id: i32,
    pub name: String,
    pub bio: String,
}

#[derive(GraphQLObject, Queryable, Identifiable, Selectable)]
#[diesel(table_name = bird)]
pub struct Bird {
    pub id: i32,
    pub common_name: String,
    pub commonwealth_status: String,
    pub nsw_status: String,
    pub profile: String,
    pub scientific_name: String,
    pub stats: String,
    pub stats_for: String,
}

#[derive(GraphQLObject, Queryable, Identifiable, Selectable)]
#[diesel(table_name = threat)]
pub struct Threat {
    pub id: i32,
    pub name: String,
}


#[derive(GraphQLObject, Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Bird))]
#[diesel(belongs_to(Threat))]
#[diesel(table_name = bird_threat)]
#[diesel(primary_key(bird_id, threat_id))]
pub struct BirdThreat {
    pub bird_id: i32,
    pub threat_id: i32,
}

#[derive(GraphQLObject)]
pub struct BirdResponse {
    pub bird: Bird,
    pub threats: Vec<Threat>,
    pub attributes: Vec<Attribute>,
}

#[derive(GraphQLInputObject, Insertable)]
#[diesel(table_name = attribute)]
pub struct AttributeInput {
    pub bird_id: i32,
    pub attributor_id: i32,
    pub bio: String,
    pub link: String,
}

#[derive(GraphQLObject)]
pub struct AttributeResponse {
    pub bird: Bird,
    pub attributor: Attributor,
    pub bio: String,
    pub link: String,
}