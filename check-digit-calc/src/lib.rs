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
}

impl TryFrom<&str> for Gtin {
	type Error = ();

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		// check length
		if value.len() != 14 {
			return Err(());
		}

		// assert that all characters are digits
		if !value
			.chars()
			.all(|c| c.is_ascii_digit())
		{
			return Err(());
		}

		let mut value =
			TryInto::<[u8; 14]>::try_into(&value.as_bytes()[..14]).unwrap();

		value
			.iter_mut()
			.for_each(|b| *b -= b'0');

		if Self::calc_check_digit(&value[..13].try_into().unwrap()) != value[13]
		{
			return Err(());
		} else {
			Ok(Self(value))
		}
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
