mod authorization;
mod common;
mod error;
mod handler;

use crate::connectors::ConnectorsBuilders;
use authorization::{professional_user_filter, public_user_filter};
use common::Context;
use error::handle_rejection;
use std::{env, net::SocketAddr};
use uuid::Uuid;
use warp::Filter;

const CONTENT_LENGTH_LIMIT: u64 = 1024 * 16;

pub async fn run(builders: ConnectorsBuilders) {
    let environment = env::var("BACKEND_ENV").expect("Missing BACKEND_ENV");

    let addr: SocketAddr = env::var("LISTEN")
        .expect("Missing LISTEN")
        .parse()
        .expect("Invalid LISTEN");

    // Prepare Context
    let context = Context { builders };
    let moved_context = context.clone();
    let context_filter = warp::any().map(move || moved_context.clone());

    // GET / -> HealthResponse
    let health = warp::get().and(warp::path::end()).map(handler::index);

    // POST /login {email, role, organization_name?} -> 200
    let login = warp::post()
        .and(warp::path!("login"))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .map(handler::authentication::login);

    // POST /logout -> 200
    let logout = warp::post()
        .and(warp::path!("logout"))
        .and(public_user_filter(context.clone()))
        .and(context_filter.clone())
        .map(handler::authentication::logout);

    // POST /session/<session_id>/validate {confirmation_token} -> Credentials
    let session_validate = warp::post()
        .and(warp::path!("session" / Uuid / "validate"))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .map(handler::authentication::validate);

    // GET /place/<id> -> Place
    let get_place = warp::get()
        .and(warp::path!("place" / Uuid))
        .and(context_filter.clone())
        .map(handler::place::get_one);

    // GET /places -> Vec<Place>
    let get_places = warp::get()
        .and(warp::path!("places"))
        .and(professional_user_filter(context.clone()))
        .and(context_filter.clone())
        .map(handler::place::get_all);

    // POST /place/<id> -> Place
    let create_place = warp::post()
        .and(warp::path!("place"))
        .and(professional_user_filter(context.clone()))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .map(handler::place::create);

    // POST /place/<id> -> Place
    let set_place = warp::post()
        .and(warp::path!("place" / Uuid))
        .and(professional_user_filter(context.clone()))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .map(handler::place::update);

    // DELETE /place/<id> -> 200
    let delete_place = warp::delete()
        .and(warp::path!("place" / Uuid))
        .and(professional_user_filter(context.clone()))
        .and(context_filter.clone())
        .map(handler::place::delete);

    // GET /profile -> Profile(Organization)
    let get_profile = warp::get()
        .and(warp::path!("profile"))
        .and(public_user_filter(context.clone()))
        .and(context_filter.clone())
        .map(handler::profile::get);

    // POST /profile {email?} -> 200
    let set_profile = warp::post()
        .and(warp::path!("profile"))
        .and(public_user_filter(context.clone()))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .map(handler::profile::update);

    // DELETE /profile -> 200
    let delete_profile = warp::delete()
        .and(warp::path!("profile"))
        .and(public_user_filter(context.clone()))
        .and(context_filter.clone())
        .map(handler::profile::delete);

    // POST /organization {name} -> 200
    let set_organization = warp::post()
        .and(warp::path!("organization"))
        .and(professional_user_filter(context.clone()))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .map(handler::organization::update);

    // POST /checkin {uuid, email, store_email, duration} -> 200
    let checkin = warp::post()
        .and(warp::path!("checkin"))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .map(handler::checkin::create);

    // GET /checkins -> Checkin(Place)
    let get_checkins = warp::get()
        .and(warp::path!("checkins"))
        .and(public_user_filter(context.clone()))
        .and(context_filter.clone())
        .map(handler::checkin::get_all);

    // POST /infection -> 200
    let create_infection = warp::post()
        .and(warp::path!("infection"))
        .and(professional_user_filter(context.clone()))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .map(handler::infection::create);

    // GET /infections -> Vec<Infection>
    let get_infections = warp::get()
        .and(warp::path!("infections"))
        .and(professional_user_filter(context.clone()))
        .and(context_filter.clone())
        .map(handler::infection::get_all);

    // Concatenate routes
    let routes = health
        .or(login)
        .or(logout)
        .or(session_validate)
        .or(get_place)
        .or(get_places)
        .or(create_place)
        .or(set_place)
        .or(delete_place)
        .or(get_profile)
        .or(set_profile)
        .or(delete_profile)
        .or(set_organization)
        .or(checkin)
        .or(get_checkins)
        .or(create_infection)
        .or(get_infections)
        .recover(handle_rejection);

    log::info!("Configured for {}", environment);
    log::info!("Listening on {}", addr);

    warp::serve(routes).run(addr).await;
}
