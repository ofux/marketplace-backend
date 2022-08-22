use crate::routes::api_key::ApiKey;
use http_api_problem::{HttpApiProblem, StatusCode};
use marketplace_core::{application::CreateContributionUsecase, github};
use marketplace_domain::*;
use rocket::{
	http::Status,
	serde::{json::Json, Deserialize},
	State,
};
use rocket_okapi::{openapi, JsonSchema};
use starknet::core::types::FieldElement;
use std::{result::Result, str::FromStr};
use uuid::Uuid;

#[derive(Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct CreateContributionDto {
	github_issue_number: u128,
	project_id: u128,
	gate: u8,
	validator: HexPrefixedString,
}

#[openapi(tag = "Contributions")]
#[post("/contributions/github", format = "application/json", data = "<body>")]
pub async fn create_contribution(
	_api_key: ApiKey,
	body: Json<CreateContributionDto>,
	github_api: &State<github::API>,
	usecase: &State<Box<dyn CreateContributionUsecase>>,
) -> Result<Status, HttpApiProblem> {
	let body = body.into_inner();
	let validator = FieldElement::from_str(&body.validator.to_string()).map_err(|e| {
		HttpApiProblem::new(StatusCode::BAD_REQUEST)
			.title("Invalid validator address")
			.detail(e.to_string())
	})?;

	let github_issue = github_api.issue(body.project_id, body.github_issue_number).await;
	let github_issue = match github_issue {
		Ok(github_issue) => github_issue,
		Err(error) =>
			return Err(HttpApiProblem::new(StatusCode::BAD_REQUEST)
				.title("Unable to get GitHub issue data")
				.detail(error.to_string())),
	};

	let metadata = github::extract_metadata(github_issue.clone());

	let contribution = Contribution {
		id: Uuid::new_v4().into(),
		onchain_id: (body.project_id * 1_000_000 + body.github_issue_number).to_string(),
		project_id: body.project_id.to_string(),
		contributor_id: None,
		title: Some(github_issue.title),
		description: Some(github_issue.body.unwrap_or_default()),
		status: ContributionStatus::Open,
		external_link: Some(github_issue.html_url),
		gate: body.gate,
		metadata,
		validator,
	};

	usecase.send_creation_request(contribution).map_err(|error| {
		HttpApiProblem::new(StatusCode::INTERNAL_SERVER_ERROR)
			.title("Unable to add contribution to the queue")
			.detail(error.to_string())
	})?;

	Ok(Status::Accepted)
}