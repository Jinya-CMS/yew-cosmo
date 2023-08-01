pub mod prelude {
    use yew::Classes;

    pub use crate::button::*;
    pub use crate::form::*;
    pub use crate::layout::*;
    pub use crate::list::*;
    pub use crate::loader::*;
    pub use crate::menu::*;
    pub use crate::modal::*;
    pub use crate::table::*;
    pub use crate::toolbar::*;
    pub use crate::typography::*;

    pub use chrono::DateTime;
    pub use chrono::NaiveTime;
    pub use chrono::Local;
    pub use chrono::NaiveDate;

    pub use color_art::Color;
    pub use color_art::color;

    #[cfg(feature = "with-icons")]
    pub use yew_icons::IconId;

    #[derive(PartialEq, Clone, Default)]
    pub enum CosmoTheme {
        #[default]
        Auto,
        Light,
        Dark,
    }

    impl ToString for CosmoTheme {
        fn to_string(&self) -> String {
            match self {
                CosmoTheme::Auto => "".to_string(),
                CosmoTheme::Light => "cosmo--light-theme".to_string(),
                CosmoTheme::Dark => "cosmo--dark-theme".to_string(),
            }
        }
    }

    impl From<CosmoTheme> for Classes {
        fn from(value: CosmoTheme) -> Self {
            value.to_string().into()
        }
    }
}

mod modal;
mod menu;
mod loader;
mod layout;
mod form;
mod list;
mod toolbar;
mod button;
mod typography;
mod table;

