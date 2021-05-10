mod common;

use super::error::{is_one, Error};
use super::organization::Organization;
use super::schema::{organization, place::dsl};
use crate::connector::Connector;
use crate::types::{Pagination, PaginationQuery};
use diesel::prelude::*;
use postgis::ewkb::Point;
use postgis_diesel::*;
use uuid::Uuid;

pub use common::*;

pub fn get(connector: &Connector, id: &Uuid) -> Result<Place, Error> {
    let connection = connector.local.pool.get()?;

    dsl::place
        .filter(dsl::id.eq(id).and(dsl::disabled.eq(false)))
        .first::<Place>(&connection)
        .map_err(|error| error.into())
}

pub fn refresh_all_gauges(connector: &Connector) -> Result<usize, Error> {
    let connection = connector.local.pool.get()?;

    diesel::sql_query(
        "UPDATE place
        SET current_gauge = checkin.active_count
        FROM (SELECT place_id, SUM(number) as active_count
            FROM checkin
            WHERE start_timestamp <= NOW() AND end_timestamp >= NOW()
            GROUP BY place_id) as checkin
        WHERE checkin.place_id = place.id",
    )
    .execute(&connection)
    .map_err(|error| error.into())
}

pub fn get_with_organization(
    connector: &Connector,
    id: &Uuid,
) -> Result<(Place, Organization), Error> {
    let connection = connector.local.pool.get()?;

    dsl::place
        .inner_join(organization::dsl::organization)
        .filter(dsl::id.eq(id).and(dsl::disabled.eq(false)))
        .first::<(Place, Organization)>(&connection)
        .map_err(|error| error.into())
}

pub fn get_all_with_organization(
    connector: &Connector,
    organization_id: &Uuid,
) -> Result<Vec<(Place, Organization)>, Error> {
    let connection = connector.local.pool.get()?;

    dsl::place
        .inner_join(organization::dsl::organization)
        .filter(
            organization::dsl::id
                .eq(organization_id)
                .and(dsl::disabled.eq(false)),
        )
        .order(dsl::created_at.desc())
        .load::<(Place, Organization)>(&connection)
        .map_err(|error| error.into())
}

pub fn search(
    connector: &Connector,
    location: PointC<Point>,
    radius_in_meters: i64,
    pagination: PaginationQuery,
) -> Result<(Pagination, Vec<PlaceSearchResult>), Error> {
    let connection = connector.local.pool.get()?;

    let query = format!(
        "
        WITH c AS (
            SELECT ST_SetSRID(ST_MakePoint({}, {}), 4326)::geography AS center
        )
        SELECT place.id,
            place.organization_id,
            place.name,
            place.description,
            place.average_duration,
            place.disabled,
            place.created_at,
            place.updated_at,
            place.maximum_gauge,
            place.address,
            place.maximum_duration,
            place.current_gauge,
            place.location,
            organization.id AS org_id,
            organization.user_id AS org_user_id,
            organization.name AS org_name,
            organization.confirmed AS org_confirmed,
            organization.disabled AS org_disabled,
            organization.updated_at AS org_updated_at,
            organization.created_at AS org_created_at,
            ST_Distance(place.location, c.center, false) AS meter_distance
        FROM place
        JOIN c ON TRUE
        INNER JOIN organization
        ON place.organization_id = organization.id
        WHERE ST_DWithin(
            place.location,
            center,
            {},
            false
        )
        ORDER BY meter_distance ASC
        LIMIT {} OFFSET {}",
        location.v.x,
        location.v.y,
        radius_in_meters,
        pagination.limit + 1,
        (pagination.page - 1) * pagination.limit
    );

    let mut places = diesel::sql_query(query).load::<PlaceSearchRow>(&connection)?;

    let next_page = if places.len() == pagination.limit + 1 {
        places.remove(places.len() - 1);
        Some(pagination.page + 1)
    } else {
        None
    };

    Ok((
        Pagination {
            page: pagination.page,
            next_page,
        },
        places.into_iter().map(|place| place.into()).collect(),
    ))
}

pub fn validate_places_owned(
    connector: &Connector,
    organization_id: &Uuid,
    places_ids: &Vec<Uuid>,
) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;
    let length = places_ids.len() as i64;

    dsl::place
        .select(diesel::dsl::count(dsl::id))
        .filter(
            dsl::organization_id
                .eq(organization_id)
                .and(dsl::id.eq_any(places_ids)),
        )
        .first(&connection)
        .map_err(|error| error.into())
        .and_then(|count: i64| {
            if count == length {
                Ok(())
            } else {
                Err(Error::NotFoundWithName {
                    name: String::from("Place"),
                })
            }
        })
}

pub fn insert(connector: &Connector, place: &PlaceInsert) -> Result<Uuid, Error> {
    let connection = connector.local.pool.get()?;

    diesel::insert_into(dsl::place)
        .values(place)
        .returning(dsl::id)
        .get_result(&connection)
        .map_err(|error| error.into())
}

pub fn update(
    connector: &Connector,
    id: &Uuid,
    organization_id: &Uuid,
    place: &PlaceUpdate,
) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;

    diesel::update(
        dsl::place.filter(
            dsl::id
                .eq(id)
                .and(dsl::organization_id.eq(organization_id))
                .and(dsl::disabled.eq(false)),
        ),
    )
    .set(place)
    .execute(&connection)
    .map_err(|error| error.into())
    .and_then(|count| is_one(count, "Place"))
}

pub fn set_disabled(
    connector: &Connector,
    id: &Uuid,
    organization_id: &Uuid,
    disabled: bool,
) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;

    diesel::update(
        dsl::place.filter(
            dsl::id
                .eq(id)
                .and(dsl::organization_id.eq(organization_id))
                .and(dsl::disabled.eq(false)),
        ),
    )
    .set(dsl::disabled.eq(disabled))
    .execute(&connection)
    .map_err(|error| error.into())
    .and_then(|count| is_one(count, "Place"))
}
