use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use itertools::Itertools;
use mapinto::ResultMapErrInto;
use uuid::Uuid;

use crate::{
	domain::*,
	infrastructure::database::{models, schema::applications, Client},
};

impl ApplicationRepository for Client {
	fn store(&self, application: Application) -> Result<(), ApplicationRepositoryError> {
		let connection = self
			.connection()
			.map_err(|e| ApplicationRepositoryError::Infrastructure(Box::new(e)))?;

		let application = models::NewApplication::from(application);
		diesel::insert_into(applications::table)
			.values(&application)
			.execute(&*connection)
			.map_err(|e| ApplicationRepositoryError::Infrastructure(Box::new(e)))?;

		Ok(())
	}

	fn find(&self, id: &ApplicationId) -> Result<Option<Application>, ApplicationRepositoryError> {
		let connection = self
			.connection()
			.map_err(|e| ApplicationRepositoryError::Infrastructure(Box::new(e)))?;

		let res = applications::dsl::applications
			.find(id.as_uuid())
			.first::<models::Application>(&*connection);

		if let Err(diesel::result::Error::NotFound) = res {
			Ok(None)
		} else {
			res.map(|a| Some(a.into())).map_err_into()
		}
	}

	fn list_by_contribution(
		&self,
		contribution_id: &ContributionId,
		contributor_id: &Option<ContributorId>,
	) -> Result<Vec<Application>, ApplicationRepositoryError> {
		let contribution_id = Uuid::from(*contribution_id);

		let connection = self
			.connection()
			.map_err(|e| ApplicationRepositoryError::Infrastructure(Box::new(e)))?;

		let mut query = applications::dsl::applications
			.filter(applications::contribution_id.eq(contribution_id))
			.into_boxed();

		if let Some(contributor_id) = contributor_id {
			query = query.filter(applications::contributor_id.eq(contributor_id.to_string()))
		}

		let applications = query
			.load::<models::Application>(&*connection)
			.map_err(ApplicationRepositoryError::from)?;

		Ok(applications.into_iter().map_into().collect())
	}
}

impl From<Application> for models::NewApplication {
	fn from(application: crate::domain::Application) -> Self {
		Self {
			id: Uuid::from(*application.id()),
			contribution_id: Uuid::from(*application.contribution_id()),
			contributor_id: application.contributor_id().to_string(),
		}
	}
}

impl From<models::Application> for Application {
	fn from(application: models::Application) -> Self {
		Self::new(
			application.id.into(),
			application.contribution_id.into(),
			ContributorId::from(application.contributor_id.as_str()),
		)
	}
}

impl From<diesel::result::Error> for ApplicationRepositoryError {
	fn from(error: diesel::result::Error) -> Self {
		match error {
			diesel::result::Error::DatabaseError(kind, _) => match kind {
				diesel::result::DatabaseErrorKind::UniqueViolation =>
					Self::AlreadyExist(Box::new(error)),
				diesel::result::DatabaseErrorKind::ForeignKeyViolation =>
					Self::InvalidEntity(Box::new(error)),
				_ => Self::Infrastructure(Box::new(error)),
			},
			diesel::result::Error::NotFound => Self::NotFound,
			_ => Self::Infrastructure(Box::new(error)),
		}
	}
}
