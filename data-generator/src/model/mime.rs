use fake::Dummy;
use serde::{Serialize, Serializer};

#[derive(Dummy, Debug)]
pub enum Mime {
	Html,
}

impl From<&Mime> for &'static str {
	fn from(value: &Mime) -> Self {
		match value {
			Mime::Html => "text/html",
		}
	}
}

impl Serialize for Mime {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: Serializer {
		serializer.serialize_str(self.into())
	}
}
