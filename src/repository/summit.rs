extern crate bcrypt;

use crate::controllers::summit::{AllSummitRegistrantQuery, CreateSummitRegistrantInput};
use crate::models::summit::SummitRegistrant;
use crate::UnwrappedPool;
use actix_web::web::{self, Json};
use diesel::prelude::*;

pub fn create_summit_registrant<'a>(
    connection: &UnwrappedPool,
    summit_registrant_data: Json<CreateSummitRegistrantInput>,
) -> Result<SummitRegistrant, diesel::result::Error> {
    use crate::schema::summit_registrant;
    let mut summit_registrant = summit_registrant_data.into_inner();

    let insert_respond = diesel::insert_into(summit_registrant::table)
        .values(summit_registrant)
        .execute(connection);

    match insert_respond {
        Err(e) => return Err(e),
        _ => {}
    }

    let final_summit_registrant = summit_registrant::table
        .order(summit_registrant::id.desc())
        .first(connection);

    final_summit_registrant
}

pub fn get_all_summit_registrants(
    connection: &UnwrappedPool,
    q: web::Query<AllSummitRegistrantQuery>,
) -> Result<Vec<SummitRegistrant>, diesel::result::Error> {
    use crate::schema::summit_registrant;

    let mut query = summit_registrant::table.into_boxed();

    if let Some(limit) = q.limit {
        query = query.limit(limit);
    };

    let results = query.load::<SummitRegistrant>(connection);

    results
}

pub fn get_summit_registrant_by_id(
    connection: &UnwrappedPool,
    summit_registrant_id: i32,
) -> Result<SummitRegistrant, diesel::result::Error> {
    use crate::schema::summit_registrant::dsl::*;

    summit_registrant
        .filter(id.eq_all(summit_registrant_id))
        .first(connection)
}
