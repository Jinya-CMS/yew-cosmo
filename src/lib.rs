pub mod prelude {
    pub use chrono::DateTime;
    pub use chrono::Local;
    pub use chrono::NaiveDate;
    pub use chrono::NaiveTime;
    pub use color_art::color;
    pub use color_art::Color;
    use yew::Classes;
    #[cfg(feature = "with-icons")]
    pub use yew_icons::IconId;

    pub use crate::button::*;
    pub use crate::form::*;
    pub use crate::layout::*;
    pub use crate::list::*;
    pub use crate::loader::*;
    pub use crate::menu::*;
    pub use crate::message::*;
    pub use crate::modal::*;
    pub use crate::tab::*;
    pub use crate::table::*;
    pub use crate::toolbar::*;
    pub use crate::typography::*;

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
                CosmoTheme::Light => "is--light".to_string(),
                CosmoTheme::Dark => "is--dark".to_string(),
            }
        }
    }

    impl From<CosmoTheme> for Classes {
        fn from(value: CosmoTheme) -> Self {
            value.to_string().into()
        }
    }
}

mod button;
mod form;
mod layout;
mod list;
mod loader;
mod menu;
mod message;
mod modal;
mod tab;
mod table;
mod toolbar;
mod typography;
