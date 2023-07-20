use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "fe/build/"]
pub struct Asset;
