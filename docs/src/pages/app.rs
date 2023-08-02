use bounce::BounceRoot;
use bounce::helmet::*;
use yew::prelude::*;
use yew_router::prelude::*;

use yew_cosmo::prelude::*;

use crate::pages::controls::dialog::Dialog;
use crate::pages::controls::html::HtmlControls;
use crate::pages::controls::side_list::SideList;
use crate::pages::controls::tab_control::TabControl;
use crate::pages::controls::toolbar::Toolbar;
use crate::pages::cosmo::about::AboutCosmo;
use crate::pages::cosmo::customize::Customize;
use crate::pages::cosmo::theme::Theme;
use crate::pages::cosmo::typography::Typography;
use crate::pages::layout::base::BaseLayout;
use crate::pages::layout::showcase::Showcase;
use crate::routing::{ControlsRoute, CosmoRoute, DocsRoute, LayoutRoute};

fn format_title(s: AttrValue) -> AttrValue {
    if s.is_empty() {
        AttrValue::from("Cosmo Yew")
    } else {
        AttrValue::from(format!("{s} – Cosmo Yew"))
    }
}

fn render_sub_menu_entry<Route>(label: impl ToString, to: Route) -> impl Fn(Route) -> Html where Route: Routable + 'static {
    move |route| {
        let is_active = route.eq(&to.clone());

        html!(
            <CosmoSubMenuItemLink<Route> to={to.clone()} label={label.to_string()} is_active={is_active} />
        )
    }
}

fn switch_sub_menu(route: DocsRoute) -> Html {
    match route {
        DocsRoute::Home | DocsRoute::CosmoRoot | DocsRoute::Cosmo => {
            html!(
                <CosmoSubMenuBar>
                    <Switch<CosmoRoute> render={render_sub_menu_entry("About Cosmo", CosmoRoute::About)} />
                    <Switch<CosmoRoute> render={render_sub_menu_entry("Typography", CosmoRoute::Typography)} />
                    <Switch<CosmoRoute> render={render_sub_menu_entry("Theme", CosmoRoute::Theme)} />
                    <Switch<CosmoRoute> render={render_sub_menu_entry("Customize", CosmoRoute::Customize)} />
                </CosmoSubMenuBar>
            )
        }
        DocsRoute::Controls | DocsRoute::ControlsRoot => {
            html!(
                <CosmoSubMenuBar>
                    <Switch<ControlsRoute> render={render_sub_menu_entry("HTML Controls", ControlsRoute::Html)} />
                    <Switch<ControlsRoute> render={render_sub_menu_entry("Side List Control", ControlsRoute::SideList)} />
                    <Switch<ControlsRoute> render={render_sub_menu_entry("Tab Control", ControlsRoute::TabControl)} />
                    <Switch<ControlsRoute> render={render_sub_menu_entry("Toolbar", ControlsRoute::Toolbar)} />
                    <Switch<ControlsRoute> render={render_sub_menu_entry("Dialogs", ControlsRoute::Dialog)} />
                </CosmoSubMenuBar>
            )
        }
        DocsRoute::Layout | DocsRoute::LayoutRoot => {
            html!(
                <CosmoSubMenuBar>
                    <Switch<LayoutRoute> render={render_sub_menu_entry("Base layout and menu", LayoutRoute::Base)} />
                    <Switch<LayoutRoute> render={render_sub_menu_entry("Layout outline", LayoutRoute::Showcase)} />
                </CosmoSubMenuBar>
            )
        }
    }
}

fn render_main_menu_entry<Route>(label: impl ToString, to: Route, active: Route) -> impl Fn(Route) -> Html where Route: Routable + 'static {
    move |route| {
        let is_active = route.eq(&active.clone());

        html!(
            <CosmoMainMenuItemLink<Route> to={to.clone()} label={label.to_string()} is_active={is_active} />
        )
    }
}

fn switch_cosmo(route: CosmoRoute) -> Html {
    match route {
        CosmoRoute::About => html!(
            <>
                <Helmet>
                    <title>{"About Cosmo"}</title>
                </Helmet>
                <AboutCosmo />
            </>
        ),
        CosmoRoute::Typography => html!(
            <>
                <Helmet>
                    <title>{"Typography"}</title>
                </Helmet>
                <Typography />
            </>
        ),
        CosmoRoute::Theme => html!(
            <>
                <Helmet>
                    <title>{"Theme"}</title>
                </Helmet>
                <Theme />
            </>
        ),
        CosmoRoute::Customize => html!(
            <>
                <Helmet>
                    <title>{"Customize"}</title>
                </Helmet>
                <Customize />
            </>
        )
    }
}

fn switch_controls(route: ControlsRoute) -> Html {
    match route {
        ControlsRoute::Html => html!(
            <>
                <Helmet>
                    <title>{"HTML Controls"}</title>
                </Helmet>
                <HtmlControls />
            </>
        ),
        ControlsRoute::SideList => html!(
            <>
                <Helmet>
                    <title>{"Side List Control"}</title>
                </Helmet>
                <SideList />
            </>
        ),
        ControlsRoute::TabControl => html!(
            <>
                <Helmet>
                    <title>{"Tab Control"}</title>
                </Helmet>
                <TabControl />
            </>
        ),
        ControlsRoute::Toolbar => html!(
            <>
                <Helmet>
                    <title>{"Toolbar"}</title>
                </Helmet>
                <Toolbar />
            </>
        ),
        ControlsRoute::Dialog => html!(
            <>
                <Helmet>
                    <title>{"Dialogs"}</title>
                </Helmet>
                <Dialog />
            </>
        ),
    }
}

fn switch_layout(route: LayoutRoute) -> Html {
    match route {
        LayoutRoute::Base => html!(
            <>
                <Helmet>
                    <title>{"Base layout and menu"}</title>
                </Helmet>
                <BaseLayout />
            </>
        ),
        LayoutRoute::Showcase => html!(
            <>
                <Helmet>
                    <title>{"Layout showcase"}</title>
                </Helmet>
                <Showcase />
            </>
        ),
    }
}

fn switch_app(route: DocsRoute) -> Html {
    match route {
        DocsRoute::Home => { html!(<Redirect<DocsRoute> to={DocsRoute::CosmoRoot} />) }
        DocsRoute::CosmoRoot => { html!(<Redirect<CosmoRoute> to={CosmoRoute::About} />) }
        DocsRoute::Cosmo => { html!(<Switch<CosmoRoute> render={switch_cosmo} />) }
        DocsRoute::ControlsRoot => { html!(<Redirect<ControlsRoute> to={ControlsRoute::Html} />) }
        DocsRoute::Controls => { html!(<Switch<ControlsRoute> render={switch_controls} />) }
        DocsRoute::LayoutRoot => { html!(<Redirect<LayoutRoute> to={LayoutRoute::Base} />) }
        DocsRoute::Layout => { html!(<Switch<LayoutRoute> render={switch_layout} />) }
    }
}

fn switch_bottom_bar(route: DocsRoute) -> Html {
    match route {
        DocsRoute::Layout | DocsRoute::LayoutRoot => {
            html!(
                <CosmoBottomBar progress_state={CosmoBottomBarProgressState::Indeterminate} progress_bottom_label="Bottom label" progress_top_label="Top label">
                    <CosmoBottomBarLeftItem>
                        <CosmoButton label="I am a button on the left" />
                        <CosmoStrong>{"Hello World!"}</CosmoStrong>
                    </CosmoBottomBarLeftItem>
                    <CosmoBottomBarRightItem>
                        <CosmoCircleButton title="I am a right button" icon={IconId::LucideLeaf} />
                    </CosmoBottomBarRightItem>
                </CosmoBottomBar>
            )
        }
        _ => { html!() }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <BrowserRouter>
            <BounceRoot>
                <CosmoPageLayout>
                    <HelmetBridge default_title="Cosmo Yew" format_title={format_title} />
                    <CosmoTopBar has_right_item={true} right_item_label="Logout">
                        <CosmoTopBarItemExternal href="https://github.com/Jinya-CMS/cosmo-css" label="Github" />
                        <CosmoTopBarItemExternal href="https://gitlab.imanuel.dev/jinya-cms/cosmo-css" label="GitLab" />
                        <CosmoTopBarItemExternal href="https://crates.io/crates/yew-cosmo" label="Crate" />
                    </CosmoTopBar>
                    <CosmoMenuBar>
                        <CosmoMainMenu>
                            <Switch<DocsRoute> render={render_main_menu_entry("Cosmo", DocsRoute::CosmoRoot, DocsRoute::Cosmo)} />
                            <Switch<DocsRoute> render={render_main_menu_entry("Controls", DocsRoute::ControlsRoot, DocsRoute::Controls)} />
                            <Switch<DocsRoute> render={render_main_menu_entry("Layout", DocsRoute::LayoutRoot, DocsRoute::Layout)} />
                        </CosmoMainMenu>
                        <Switch<DocsRoute> render={switch_sub_menu} />
                    </CosmoMenuBar>
                    <CosmoPageBody>
                        <Switch<DocsRoute> render={switch_app} />
                    </CosmoPageBody>
                    <Switch<DocsRoute> render={switch_bottom_bar} />
                </CosmoPageLayout>
            </BounceRoot>
        </BrowserRouter>
    )
}
