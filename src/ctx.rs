use std::sync::Arc;

use console::StyledObject;
use once_cell::sync::Lazy;

use crate::colors::Colors;

#[derive(Default)]
pub struct Context {
    pub colors: Colors,
}

static STATIC_INSTANCE: Lazy<arc_swap::ArcSwap<Context>> =
    Lazy::new(|| arc_swap::ArcSwap::from_pointee(Context::default()));

pub fn instance() -> Arc<Context> {
    STATIC_INSTANCE.load().clone()
}

impl Context {
    pub fn aikup_label(&self) -> StyledObject<&str> {
        self.colors.label("aikup:")
    }
}
