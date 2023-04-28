use std::collections::HashMap;

use crate::custom_errors::common::Error;

#[derive(Debug, Default)]
pub struct Pagination {
    pub limit: Option<i16>,
    pub offset: i16,
}

pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<i16>()
                    .map_err(|e| Error::ParseError(e))?,
            ),
            offset: params
                .get("offset")
                .unwrap()
                .parse::<i16>()
                .map_err(|e| Error::ParseError(e))?,
        })
    }
    Err(Error::MissingParameters)
}