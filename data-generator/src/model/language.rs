use fake::Dummy;
use serde::{Serialize, Serializer};
use strum::IntoStaticStr;

#[derive(IntoStaticStr, Dummy, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum Language {
	Zh,
	Ko,
	Eo,
	En,
	Fr,
	De,
	Ja,
}

impl Serialize for Language {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: Serializer {
		serializer.serialize_str(self.into())
	}
}
