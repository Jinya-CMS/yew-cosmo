use yew_router::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum DocsRoute {
    #[at("/")]
    #[not_found]
    Home,
    #[at("/cosmo")]
    CosmoRoot,
    #[at("/cosmo/*")]
    Cosmo,
}

#[derive(Routable, Clone, PartialEq)]
pub enum CosmoRoute {
    #[at("/cosmo/about")]
    About,
    #[at("/cosmo/typography")]
    Typography,
    #[at("/cosmo/theme")]
    Theme,
    #[at("/cosmo/customize")]
    Customize,
}