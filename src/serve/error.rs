use custom_error::custom_error;
use serde::Serialize;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::{reject, Rejection, Reply};

custom_error! { pub Error
    InvalidData = "Invalid data",
    Unauthorized = "Unauthorized",
    // ModelError {source: model::error::Error} = "[Model] {source}",
}

impl reject::Reject for Error {}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

struct InternalErrorResponse {
    code: StatusCode,
    message: String,
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    let response: Result<InternalErrorResponse, Rejection>;

    if let Some(error) = err.find::<Error>() {
        response = Ok(InternalErrorResponse {
            code: match error {
                Error::InvalidData => StatusCode::BAD_REQUEST,
                Error::Unauthorized => StatusCode::UNAUTHORIZED,
                // Error::ModelError { source } => match source {
                //     Model::error::Error::ModelNotFound => StatusCode::NOT_FOUND,
                //     _ => StatusCode::INTERNAL_SERVER_ERROR,
                // },
            },
            message: error.to_string(),
        });
    } else if let Some(missing_header) = err.find::<warp::reject::MissingHeader>() {
        if missing_header.name() == "authorization" {
            response = Ok(InternalErrorResponse {
                code: StatusCode::UNAUTHORIZED,
                message: String::from("Unauthorized"),
            })
        } else {
            response = Err(err);
        }
    } else {
        response = Err(err);
    }

    response.map(|error| {
        warp::reply::with_status(
            warp::reply::json(&ErrorResponse {
                code: error.code.as_u16(),
                message: error.message,
            }),
            error.code,
        )
    })
}
