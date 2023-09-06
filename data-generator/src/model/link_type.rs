use std::fmt::Display;

use fake::Dummy;
use serde::{Serialize, Serializer};
use strum::IntoStaticStr;

#[derive(IntoStaticStr, Dummy, Debug)]
#[strum(serialize_all = "camelCase")]
pub enum LinkType {
	ActivityIdeas,
	AllergenInfo,
	BrandHomepageClinical,
	BrandHomepagePatient,
	CareersInfo,
	CertificationInfo,
	ConsumerHandlingStorageInfo,
	DefaultLink,
	DefaultLinkMulti,
	Epcis,
	Epil,
	EventsInfo,
	Faqs,
	HandledBy,
	HasRetailers,
	Homepage,
	IngredientsInfo,
	Instructions,
	Jws,
	LeaveReview,
	LocationInfo,
	LogisticsInfo,
	MasterData,
	MenuInfo,
	NutritionalInfo,
	OpeningHoursInfo,
	PaymentLink,
	Pip,
	ProductSustainabilityInfo,
	Promotion,
	PurchaseSuppliesOrAccessories,
	QuickStartGuide,
	RecallStatus,
	RecipeInfo,
	RegisterProduct,
	RegistryEntry,
	RelatedVideo,
	Review,
	SafetyInfo,
	ScheduleTime,
	ServiceInfo,
	SmartLabel,
	Smpc,
	SocialMedia,
	StatisticInfo,
	Support,
	SustainabilityInfo,
	Traceability,
	Tutorial,
	UserAgreement,
	VerificationService,
	WhatsInTheBox,
}

impl Display for LinkType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let camel: &'static str = self.into();
		write!(f, "gs1:{camel}")
	}
}

impl Serialize for LinkType {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: Serializer {
		serializer.serialize_str(&self.to_string())
	}
}
