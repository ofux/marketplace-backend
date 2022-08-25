#[cfg(test)]
mod tests;

use http_api_problem::HttpApiProblem;
use itertools::Itertools;
use marketplace_core::dto;
use marketplace_domain::ApplicationRepository;
use rocket::{serde::json::Json, State};
use rocket_okapi::openapi;
use std::sync::Arc;

use crate::routes::{hex_string::HexString, to_http_api_problem::ToHttpApiProblem};

#[openapi(tag = "Applications")]
#[get("/applications?<contributor_id>")]
pub async fn list_contributor_applications(
	contributor_id: Option<HexString>,
	application_repository: &State<Arc<dyn ApplicationRepository>>,
) -> Result<Json<Vec<dto::Application>>, HttpApiProblem> {
	let contributor_id = contributor_id.map(|id| id.into());

	let applications = application_repository
		.list_by_contributor(contributor_id)
		.map_err(|e| e.to_http_api_problem())?;

	Ok(Json(applications.into_iter().map_into().collect()))
}
