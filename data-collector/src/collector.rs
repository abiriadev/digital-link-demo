use anyhow::Ok;
use indicatif::{ParallelProgressIterator, ProgressBar};
use rayon::prelude::{
	IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};

use crate::models::{catalog::CatalogRequest, ProductRequest, ResolveRequest};

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

	pub fn resolve(&self, categories: &[&str]) -> Vec<ProductRequest> {
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
					.map(|a| a)
			})
			.flatten()
			.collect::<Vec<_>>()
	}
}
