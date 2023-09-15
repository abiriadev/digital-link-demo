use anyhow::Ok;
use indicatif::{ParallelProgressIterator, ProgressBar};
use rayon::prelude::{
	IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};

use crate::models::{
	catalog::CatalogRequest,
	product::{Product, ResolverEntry},
	ResolveRequest,
};

#[derive(Debug)]
pub struct Collector {
	base_url: String,
	pb: ProgressBar,
}

impl Collector {
	pub fn new(base_url: String) -> anyhow::Result<Self> {
		Ok(Self {
			base_url,
			pb: ProgressBar::new(0),
		})
	}

	pub fn resolve(&self, categories: &[&str]) -> Vec<Product> {
		self.pb
			.set_length(categories.len() as u64);

		categories
			.par_iter()
			.map(|&s| CatalogRequest::from(s))
			.map(|cr| cr.resolve(&self.base_url).unwrap())
			.progress_with(self.pb.clone())
			.map(|c| {
				c.product_request
					.into_par_iter()
					.map(|p| Product::resolve(p, &self.base_url))
			})
			.flatten()
			.collect::<Vec<_>>()
	}

	pub fn serialize(result: Vec<Product>) -> serde_json::Result<String> {
		serde_json::to_string(
			&result
				.into_iter()
				.map(ResolverEntry::from)
				.collect::<Vec<_>>(),
		)
	}
}
