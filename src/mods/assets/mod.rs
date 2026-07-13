use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets/"]
pub struct Assets;

impl Assets {
    #[tracing::instrument(skip_all)]
    pub fn new() -> Self {
        Self
    }
}
