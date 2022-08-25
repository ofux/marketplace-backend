#[cfg(test)]
mod tests;

use std::str::FromStr;

use marketplace_domain::{ContributorId, HexPrefixedString, ParseHexPrefixedStringError};
use marketplace_wrappers::HexStringWrapper;
use okapi::openapi3::SchemaObject;
use rocket::{
	data::ToByteUnit,
	form::{self, DataField, FromFormField, ValueField},
	request::FromParam,
};
use schemars::{
	schema::{InstanceType, StringValidation},
	JsonSchema,
};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Hash, HexStringWrapper)]
pub struct HexString(pub HexPrefixedString);

impl JsonSchema for HexString {
	fn schema_name() -> String {
		"HexString".to_string()
	}

	fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
		let schema = SchemaObject {
			instance_type: Some(InstanceType::String.into()),
			string: Some(Box::new(StringValidation {
				min_length: Some(3),
				max_length: Some(66),
				pattern: Some("\\b0x[0-9a-f]+\\b".to_string()),
			})),
			..Default::default()
		};

		schema.into()
	}
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for HexString {
	fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
		HexString::from_str(field.value).map_err(|parse_error| match parse_error {
			ParseHexPrefixedStringError::TooShort => form::error::ErrorKind::InvalidLength {
				min: Some(3),
				max: Some(66),
			}
			.into(),
			ParseHexPrefixedStringError::InvalidPrefix => {
				form::Error::validation(parse_error.to_string()).into()
			},
			ParseHexPrefixedStringError::InvalidHexa(_) => {
				form::Error::validation(parse_error.to_string()).into()
			},
		})
	}

	async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
		let limit = 256.kibibytes();
		let bytes = field.data.open(limit).into_bytes().await?;
		if !bytes.is_complete() {
			Err((None, Some(limit)))?;
		}

		let bytes = bytes.into_inner();
		let bytes = rocket::request::local_cache!(field.request, bytes);
		let s = std::str::from_utf8(bytes)?;

		HexString::from_str(s).map_err(|e| match e {
			ParseHexPrefixedStringError::TooShort => form::error::ErrorKind::InvalidLength {
				min: Some(3),
				max: Some(66),
			}
			.into(),
			ParseHexPrefixedStringError::InvalidPrefix => {
				form::Error::validation(e.to_string()).into()
			},
			ParseHexPrefixedStringError::InvalidHexa(_) => {
				form::Error::validation(e.to_string()).into()
			},
		})
	}
}

impl<'a> FromParam<'a> for HexString {
	type Error = <HexString as FromStr>::Err;

	fn from_param(param: &'a str) -> Result<Self, Self::Error> {
		HexString::from_str(param)
	}
}

impl From<HexString> for HexPrefixedString {
	fn from(value: HexString) -> Self {
		value.0
	}
}

impl From<HexString> for ContributorId {
	fn from(param: HexString) -> Self {
		param.0.into()
	}
}
