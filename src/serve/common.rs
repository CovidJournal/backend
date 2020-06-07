use crate::connectors::ConnectorsBuilders;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone)]
pub struct Context {
    pub builders: ConnectorsBuilders,
}

pub struct PublicUser {
    pub id: Uuid,
}

pub struct ProfessionalUser {
    pub id: Uuid,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub healthy: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanQuery {
    pub place_id: Uuid,
}

#[derive(Serialize)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Place {
    pub id: Uuid,
    pub organization: Organization,
    pub name: String,
    pub description: Option<String>,
    /// Average duration in minutes
    pub average_duration: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckinForm {
    pub place_id: Uuid,
    pub email: String,
    pub store_email: bool,
    pub duration: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateDeviceForm {
    pub device_id: Uuid,
    pub confirmation_token: String,
}

#[derive(Serialize)]
pub struct Credentials {
    pub login: String,
    pub token: String,
}

#[derive(Serialize)]
pub struct Profile {
    pub id: Uuid,
    pub email: Option<String>,
    pub organization: Option<Organization>,
}

#[derive(Deserialize)]
pub struct ProfileForm {
    pub email: Option<String>,
}

#[derive(Deserialize)]
pub struct OrganizationForm {
    pub name: String,
}
