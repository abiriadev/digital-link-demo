use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "product.stpl")]
pub struct ProductTemplate {
	pub host: String,
	pub pid: String,
}

#[derive(TemplateOnce)]
#[template(path = "404.stpl")]
pub struct NotFoundTemplate {
	pub link_types: Vec<&'static str>,
}
