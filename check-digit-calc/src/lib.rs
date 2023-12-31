use core::fmt;
use std::{
	fmt::{Display, Formatter},
	str::from_utf8_unchecked,
};

use rand::{thread_rng, Rng};
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
	#[error("GTIN should be 14 characters long")]
	LengthDoesNotMatch,
	#[error("all characters should be digits")]
	NonDigitCharacter,
	#[error("check digit does not match. correct digit should be: {0}")]
	CheckDigitDoesNotMatch(u8),
}

pub struct Gtin([u8; 14]);

impl Gtin {
	fn difference_between_nearest_multiple_of_ten(n: u8) -> u8 {
		(10 - n % 10) % 10
	}

	fn calc_check_digit(body: &[u8; 13]) -> u8 {
		Self::difference_between_nearest_multiple_of_ten(
			body.into_iter()
				.enumerate()
				.map(|(i, &n)| if i % 2 == 0 { 3 * n } else { n })
				.sum(),
		)
	}

	pub fn calc_check_digit_from_str(
		value: &str,
	) -> Result<char, ValidationError> {
		// check length
		if value.len() != 13 {
			return Err(ValidationError::LengthDoesNotMatch);
		}

		// assert that all characters are digits
		if !value
			.chars()
			.all(|c| c.is_ascii_digit())
		{
			return Err(ValidationError::NonDigitCharacter);
		}

		let a: [u8; 13] = value.as_bytes().try_into().unwrap();

		Ok((Self::calc_check_digit(&a.map(|b| b - b'0')) + b'0') as char)
	}

	pub fn generate() -> Self { Self::generate_with_rng(thread_rng()) }

	pub fn generate_with_rng<R: Rng>(mut rng: R) -> Self {
		let mut buf: [u8; 14] = [0; 14];

		rng.fill(&mut buf[..13]);
		buf.iter_mut()
			.for_each(|b| *b = rng.gen_range(0..10));

		buf[13] = Self::calc_check_digit(&buf[..13].try_into().unwrap());

		Self(buf)
	}

	pub fn generate_with_seed(seed: &str) -> Self {
		Self::generate_with_rng(Seeder::from(seed).make_rng::<Pcg64>())
	}
}

impl TryFrom<&str> for Gtin {
	type Error = ValidationError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		// check length
		if value.len() != 14 {
			return Err(ValidationError::LengthDoesNotMatch);
		}

		// assert that all characters are digits
		if !value
			.chars()
			.all(|c| c.is_ascii_digit())
		{
			return Err(ValidationError::NonDigitCharacter);
		}

		let mut value =
			TryInto::<[u8; 14]>::try_into(&value.as_bytes()[..14]).unwrap();

		value
			.iter_mut()
			.for_each(|b| *b -= b'0');

		let check = Self::calc_check_digit(&value[..13].try_into().unwrap());
		if check != value[13] {
			return Err(ValidationError::CheckDigitDoesNotMatch(
				check,
			));
		}

		Ok(Self(value))
	}
}

impl Display for Gtin {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", unsafe {
			from_utf8_unchecked(&self.0.map(|b| b + b'0')[..])
		})
	}
}

#[test]
fn test_difference_between_nearest_multiple_of_ten() {
	assert_eq!(
		Gtin::difference_between_nearest_multiple_of_ten(0),
		0
	);
	assert_eq!(
		Gtin::difference_between_nearest_multiple_of_ten(1),
		9
	);
	assert_eq!(
		Gtin::difference_between_nearest_multiple_of_ten(2),
		8
	);
	assert_eq!(
		Gtin::difference_between_nearest_multiple_of_ten(3),
		7
	);
	assert_eq!(
		Gtin::difference_between_nearest_multiple_of_ten(4),
		6
	);
	assert_eq!(
		Gtin::difference_between_nearest_multiple_of_ten(5),
		5
	);
	assert_eq!(
		Gtin::difference_between_nearest_multiple_of_ten(6),
		4
	);
	assert_eq!(
		Gtin::difference_between_nearest_multiple_of_ten(7),
		3
	);
	assert_eq!(
		Gtin::difference_between_nearest_multiple_of_ten(8),
		2
	);
	assert_eq!(
		Gtin::difference_between_nearest_multiple_of_ten(9),
		1
	);
	assert_eq!(
		Gtin::difference_between_nearest_multiple_of_ten(10),
		0
	);
	assert_eq!(
		Gtin::difference_between_nearest_multiple_of_ten(11),
		9
	);
}
