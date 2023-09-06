use std::fmt::Display;

use fake::Dummy;
use strum::IntoStaticStr;

#[derive(IntoStaticStr, Dummy, Debug)]
#[strum(serialize_all = "camelCase")]
pub enum LinkType {
	ActivityIdeasgs1,
	AllergenInfogs1,
	BrandHomepageClinicalgs1,
	BrandHomepagePatientgs1,
	CareersInfogs1,
	CertificationInfogs1,
	ConsumerHandlingStorageInfogs1,
	DefaultLinkgs1,
	DefaultLinkMultigs1,
	Epcisgs1,
	Epilgs1,
	EventsInfogs1,
	Faqsgs1,
	HandledBygs1,
	HasRetailersgs1,
	Homepagegs1,
	IngredientsInfogs1,
	Instructionsgs1,
	Jwsgs1,
	LeaveReviewgs1,
	LocationInfogs1,
	LogisticsInfogs1,
	MasterDatags1,
	MenuInfogs1,
	NutritionalInfogs1,
	OpeningHoursInfogs1,
	PaymentLinkgs1,
	Pipgs1,
	ProductSustainabilityInfogs1,
	Promotiongs1,
	PurchaseSuppliesOrAccessoriesgs1,
	QuickStartGuidegs1,
	RecallStatusgs1,
	RecipeInfogs1,
	RegisterProductgs1,
	RegistryEntrygs1,
	RelatedVideogs1,
	Reviewgs1,
	SafetyInfogs1,
	ScheduleTimegs1,
	ServiceInfogs1,
	SmartLabelgs1,
	Smpcgs1,
	SocialMediags1,
	StatisticInfogs1,
	Supportgs1,
	SustainabilityInfogs1,
	Traceabilitygs1,
	Tutorialgs1,
	UserAgreementgs1,
	VerificationServicegs1,
	WhatsInTheBoxgs1,
}

impl Display for LinkType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "gs1:{self}")
	}
}
