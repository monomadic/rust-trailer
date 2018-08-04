use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;

use ::error::*;

pub fn handle_request(result: Result<String, ServerError>) -> IronResult<Response> {
    match result {
        Ok(body) => Ok(Response::with((
            ContentType::html().0,
            status::Ok,
            body,
        ))),
        Err(error) => Err(::iron::error::IronError {
            error: Box::new(error),
            response: Response::with((
                ContentType::html().0,
                status::Ok,
                "iron error".to_string(),
            )),
        })
    }
}

pub fn funds(_req: &mut Request) -> Result<String, ServerError> {
    // let coins = ::models::Coin::all(&db)?;
    // let view = ::views::landing(coins)?;
    Ok("view".to_string())
}
