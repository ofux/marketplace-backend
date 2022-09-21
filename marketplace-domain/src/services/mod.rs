mod contact_information;
pub use contact_information::{
	ContactInformationService as ContactInformationServiceImplementation,
	MockService as MockContactInformationService, Service as ContactInformationService,
};

mod onchain_contribution;
pub use onchain_contribution::{
	Error as OnchainContributionServiceError, MockService as MockOnchainContributionService,
	Service as OnchainContributionService,
};

mod uuid;
pub use self::uuid::{
	MockService as MockUuidGenerator, RandomUuidGenerator, Service as UuidGenerator,
};

mod github_client;
pub use github_client::{Error as GithubClientError, GithubClient};

mod contributor;
pub use contributor::{Error as ContributorServiceError, Service as ContributorService};

mod onchain_account_verifier;
pub use onchain_account_verifier::{Error as OnChainAccountVerifierError, OnChainAccountVerifier};

#[cfg(test)]
pub use github_client::MockGithubClient;

#[cfg(test)]
pub use contributor::MockService as MockContributorService;
