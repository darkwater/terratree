use egui::{
    generate_loader_id,
    load::{Bytes, BytesLoadResult, BytesLoader, BytesPoll, LoadError},
    Vec2,
};

pub struct Loader {}

impl Loader {
    pub fn new() -> Self {
        Self {}
    }
}

impl BytesLoader for Loader {
    fn id(&self) -> &str {
        generate_loader_id!(Loader)
    }

    fn load(&self, _: &egui::Context, uri: &str) -> BytesLoadResult {
        let Some(name) = uri.strip_prefix("wiki://") else {
            return Err(LoadError::NotSupported);
        };

        wiki_data::IMAGES
            .iter()
            .find(|i| i.name == name)
            .map_or_else(
                || Err(LoadError::Loading("File not found".to_string())),
                |i| {
                    Ok(BytesPoll::Ready {
                        size: Some(Vec2::new(i.width as f32, i.height as f32)),
                        bytes: Bytes::Static(i.data),
                        mime: None,
                    })
                },
            )
    }

    fn forget(&self, _: &str) {
        // no cache to manage
    }

    fn forget_all(&self) {
        // no cache to manage
    }

    fn byte_size(&self) -> usize {
        0
    }
}
