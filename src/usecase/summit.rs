use crate::{
    controllers::summit::{
        AllSummitRegistrantQuery, CreateSummitRegistrantInput, GetSummitRegistrantPath,
    },
    models::summit::SummitRegistrant,
    repository, Pool,
};
use actix_web::{
    error::ErrorBadRequest,
    web::{self, Json},
};

pub fn get_all_summit_registrants(
    db: web::Data<Pool>,
    query: web::Query<AllSummitRegistrantQuery>,
) -> Result<Vec<SummitRegistrant>, actix_web::Error> {
    repository::summit::get_all_summit_registrants(&db.get().unwrap(), query)
        .map(|all_user| all_user)
        .map_err(|err| ErrorBadRequest(err))
}

pub fn get_summit_registrant_by_id(
    db: web::Data<Pool>,
    info: web::Path<GetSummitRegistrantPath>,
) -> Result<SummitRegistrant, actix_web::Error> {
    repository::summit::get_summit_registrant_by_id(&db.get().unwrap(), info.id)
        .map(|user| user)
        .map_err(|err| ErrorBadRequest(err))
}

pub fn create_summit_registrant(
    db: web::Data<Pool>,
    user: Json<CreateSummitRegistrantInput>,
) -> Result<SummitRegistrant, actix_web::Error> {
    repository::summit::create_summit_registrant(&db.get().unwrap(), user)
        .map(|user| user)
        .map_err(|err| ErrorBadRequest(err))
}
