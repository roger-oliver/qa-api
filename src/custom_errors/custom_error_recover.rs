use warp::{reject::Rejection, Reply, reply::with_status, hyper::StatusCode};

use super::{store::Error, account};

// all postgres errors should be treated as string.
// that is lucky that the 23 category is only numbers
// but see this example: 42P01	UNDEFINED TABLE;
const DUPLICATE_KEY: &str = "23505";

pub async fn return_custom_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(Error::DatabaseQueryError(e)) = r.find() {
        match e {
            sqlx::Error::Database(err) => {
                if err.code().unwrap() == DUPLICATE_KEY {
                    Ok(with_status(
                        "Entry already exists",
                        StatusCode::UNPROCESSABLE_ENTITY
                    ))
                } else {
                    Ok(with_status(
                        "Cannot process data: {err}",
                        StatusCode::UNPROCESSABLE_ENTITY
                    ))
                }
            }
            sqlx::Error::RowNotFound => {
                Ok(with_status("Entry not found", StatusCode::NOT_FOUND))
            }
            _ => {
                    Ok(with_status(
                        "Cannot process data: {err}",
                        StatusCode::UNPROCESSABLE_ENTITY
                    ))
            }
        }
    // custom return error when user gives a wrong credentials
    } else if let Some(account::Error::WrongCredentials) = r.find() {
        Ok(with_status("invalid credentials", StatusCode::UNAUTHORIZED))
    } else if let Some(account::Error::Unauthorized) = r.find() {
        Ok(with_status("you are not authorised", StatusCode::UNAUTHORIZED))
    } else {

        if r.is_not_found() {
            Ok(with_status(
                "not found", 
                StatusCode::NOT_FOUND
            ))
        } else {
            Ok(with_status(
                "Wrong request",
                StatusCode::BAD_REQUEST
            ))
        }
    }
}