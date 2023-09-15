use diesel::prelude::*;
use juniper::{EmptySubscription, FieldResult, graphql_object, RootNode};

use crate::{database::Database, model::*, schema::*};

impl juniper::Context for Database {}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Database>>;

pub struct Query;

#[graphql_object(context = Database)]
impl Query {
    fn birds(#[graphql(context)] database: &mut Database) -> FieldResult<Vec<Bird>> {
        use crate::schema::bird::dsl::*;
        use diesel::RunQueryDsl;
        let connection = &mut database.pool.get()?;
        let bird_response = bird.load::<Bird>(connection)?;
        Ok(bird_response)
    }

    fn bird(
        #[graphql(context)] database: &mut Database,
        #[graphql(description = "id of the bird")] search_id: i32,
    ) -> FieldResult<BirdResponse> {
        let connection = &mut database.pool.get()?;
        let bird_response = bird::table.find(&search_id).first(connection)?;
        let bird_threats = bird_threat::table
            .filter(bird_threat::bird_id.eq(&search_id))
            .select(bird_threat::threat_id)
            .load::<i32>(connection)?;
        let threats_response = threat::table
            .filter(threat::id.eq_any(&bird_threats))
            .load::<Threat>(connection)?;
        let bird_attributes = attribute::table
            .filter(attribute::bird_id.eq(&search_id))
            .load::<Attribute>(connection)?;

        Ok(BirdResponse {
            bird: bird_response,
            threats: threats_response,
            attributes: bird_attributes,
        })
    }
}

pub struct Mutation;

#[graphql_object(context = Database)]
impl Mutation {
    fn new_attribute(
        #[graphql(context)] database: &mut Database,
        attribute_input: AttributeInput,
    ) -> FieldResult<AttributeResponse> {
        let connection = &mut database.pool.get()?;

        diesel::insert_into(attribute::table)
            .values(&attribute_input)
            .execute(connection)?;

        let bird_response = bird::table
            .find(&attribute_input.bird_id)
            .first(connection)?;

        let attributor_response = attributor::table
            .find(&attribute_input.attributor_id)
            .first(connection)?;

        Ok(AttributeResponse {
            bird: bird_response,
            attributor: attributor_response,
            bio: attribute_input.bio,
            link: attribute_input.link,
        })
    }

    fn remove_attribute(
        #[graphql(context)] database: &mut Database,
        attribute_id: i32,
    ) -> FieldResult<String> {
        let connection = &mut database.pool.get()?;
        diesel::delete(attribute::table.filter(attribute::id.eq(attribute_id)))
            .execute(connection)?;
        Ok("Attribute deleted successfully".to_string())
    }
}
