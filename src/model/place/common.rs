use super::super::schema::place;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Queryable)]
pub struct Place {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub average_duration: i32,
    pub disabled: bool,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub maximum_gauge: Option<i32>,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub maximum_duration: i32,
}

#[derive(Insertable)]
#[table_name = "place"]
pub struct PlaceInsert {
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub average_duration: i32,
    pub maximum_gauge: Option<i32>,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub maximum_duration: i32,
}

#[derive(AsChangeset)]
#[table_name = "place"]
#[changeset_options(treat_none_as_null = "true")]
pub struct PlaceUpdate {
    pub name: String,
    pub description: Option<String>,
    pub average_duration: i32,
    pub maximum_gauge: Option<i32>,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub maximum_duration: i32,
}
