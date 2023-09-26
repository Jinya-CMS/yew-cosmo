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
    #[at("/controls")]
    ControlsRoot,
    #[at("/controls/*")]
    Controls,
    #[at("/layout")]
    LayoutRoot,
    #[at("/layout/*")]
    Layout,
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

#[derive(Routable, Clone, PartialEq)]
pub enum ControlsRoute {
    #[at("/controls/html")]
    Html,
    #[at("/controls/message")]
    Message,
    #[at("/controls/side-list")]
    SideList,
    #[at("/controls/tab-control")]
    TabControl,
    #[at("/controls/toolbar")]
    Toolbar,
    #[at("/controls/dialog")]
    Dialog,
}

#[derive(Routable, Clone, PartialEq)]
pub enum LayoutRoute {
    #[at("/layout/base")]
    Base,
    #[at("/layout/showcase")]
    Showcase,
}
