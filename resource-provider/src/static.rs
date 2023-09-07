use maplit::hashmap;

use crate::LINK_TYPE;

pub fn init_once() {
	LINK_TYPE
		.set(hashmap! {
			"gs1:pip" => hashmap! {
				"en" => "Product information",
				"ko" => "제품 정보",
			},
			"gs1:quickStartGuide" => hashmap! {
				"en" => "Quick start guide",
				"ko" => "빠른 사용법",
			},
			"gs1:whatsInTheBox" => hashmap! {
				"en" => "What’s in the box",
				"ko" => "제품 구성물 목록",
			},
			"gs1:certificationInfo" => hashmap! {
				"en" => "Certification information",
				"ko" => "인증 정보",
			},
			"gs1:support" => hashmap! {
				"en" => "Support",
				"ko" => "지원 정보",
			},
		})
		.unwrap();
}
