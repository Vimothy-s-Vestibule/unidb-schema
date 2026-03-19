use diesel::prelude::*;

use serde::{Deserialize, Serialize};

use crate::models::newtypes::TextVec;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    QueryableByName,
    Default,
)]
#[diesel(table_name = crate::diesel_schema::scores)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PersonalityTraits {
    pub honesty_humility: f64,
    pub emotionality: f64,
    pub extraversion: f64,
    pub agreeableness: f64,
    pub conscientiousness: f64,
    pub openness_to_experience: f64,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    QueryableByName,
    Default,
)]
#[diesel(table_name = crate::diesel_schema::scores)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CommunicationTraits {
    pub agency: f64,
    pub communion: f64,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    QueryableByName,
    Default,
)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::diesel_schema::scores)]
pub struct PersonalityValues {
    pub self_direction: f64,
    pub stimulation: f64,
    pub hedonism: f64,
    pub achievement: f64,
    pub power: f64,
    pub security: f64,
    pub conformity: f64,
    pub tradition: f64,
    pub benevolence: f64,
    pub universalism: f64,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    QueryableByName,
    Default,
)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::diesel_schema::scores)]
pub struct PersonalityInterests {
    pub domains: TextVec,
    pub activities: TextVec,
}
