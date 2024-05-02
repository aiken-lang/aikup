use console::{Style, StyledObject};

pub struct Colors {
    version_text: Style,
    warning_text: Style,
    success_text: Style,
    info_text: Style,
    label: Style,
}

impl Default for Colors {
    fn default() -> Self {
        Self::new()
    }
}

impl Colors {
    pub fn new() -> Self {
        Self {
            version_text: Style::new().cyan(),
            warning_text: Style::new().yellow(),
            success_text: Style::new().green(),
            info_text: Style::new().blue(),
            label: Style::new().magenta().bold(),
        }
    }

    pub fn version_text<D>(&self, val: D) -> StyledObject<D> {
        self.version_text.apply_to(val)
    }

    pub fn warning_text<D>(&self, val: D) -> StyledObject<D> {
        self.warning_text.apply_to(val)
    }

    pub fn success_text<D>(&self, val: D) -> StyledObject<D> {
        self.success_text.apply_to(val)
    }

    pub fn info_text<D>(&self, val: D) -> StyledObject<D> {
        self.info_text.apply_to(val)
    }

    pub fn label<D>(&self, val: D) -> StyledObject<D> {
        self.label.apply_to(val)
    }
}
