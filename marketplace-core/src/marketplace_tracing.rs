use opentelemetry::sdk::trace::{self, RandomIdGenerator, Sampler};
use opentelemetry_datadog::ApiVersion;
use tracing_subscriber::{layer::SubscriberExt, Registry};

pub fn setup_tracing() {
	// Install a new OpenTelemetry trace pipeline
	let tracer = opentelemetry_datadog::new_pipeline()
		.with_service_name("poc-tracing")
		.with_version(ApiVersion::Version05)
		.with_agent_endpoint("http://localhost:8126")
		.with_trace_config(
			trace::config()
				.with_sampler(Sampler::AlwaysOn)
				.with_id_generator(RandomIdGenerator::default()),
		)
		.install_batch(opentelemetry::runtime::Tokio)
		.unwrap();

	// Create a tracing layer with the configured tracer
	let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

	// Use the tracing subscriber `Registry`, or any other subscriber
	// that impls `LookupSpan`
	let subscriber = Registry::default().with(telemetry);

	// Trace executed code
	tracing::subscriber::set_global_default(subscriber).unwrap();
}
