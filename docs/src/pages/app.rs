use bounce::BounceRoot;
use bounce::helmet::*;
use yew::prelude::*;
use yew_router::prelude::*;

use yew_cosmo::prelude::*;

use crate::pages::cosmo::about::AboutCosmo;
use crate::pages::cosmo::customize::Customize;
use crate::pages::cosmo::theme::Theme;
use crate::pages::cosmo::typography::Typography;
use crate::routing::{CosmoRoute, DocsRoute};

fn format_title(s: AttrValue) -> AttrValue {
    if s.is_empty() {
        AttrValue::from("Cosmo Yew")
    } else {
        AttrValue::from(format!("{s} – Cosmo Yew"))
    }
}

fn render_about_cosmo_sub_menu_entry(label: AttrValue, to: CosmoRoute) -> impl Fn(CosmoRoute) -> Html {
    move |route| {
        let is_active = route.eq(&to.clone());

        html!(
            <CosmoSubMenuItemLink<CosmoRoute> to={to.clone()} label={label.clone()} is_active={is_active} />
        )
    }
}

fn switch_sub_menu(route: DocsRoute) -> Html {
    match route {
        DocsRoute::Home | DocsRoute::CosmoRoot | DocsRoute::Cosmo => {
            html!(
                <CosmoSubMenuBar>
                    <Switch<CosmoRoute> render={render_about_cosmo_sub_menu_entry(AttrValue::from("About Cosmo"), CosmoRoute::About)} />
                    <Switch<CosmoRoute> render={render_about_cosmo_sub_menu_entry(AttrValue::from("Typography"), CosmoRoute::Typography)} />
                    <Switch<CosmoRoute> render={render_about_cosmo_sub_menu_entry(AttrValue::from("Theme"), CosmoRoute::Theme)} />
                    <Switch<CosmoRoute> render={render_about_cosmo_sub_menu_entry(AttrValue::from("Customize"), CosmoRoute::Customize)} />
                </CosmoSubMenuBar>
            )
        }
    }
}

fn render_cosmo_main_menu_entry() -> impl Fn(DocsRoute) -> Html {
    move |route| {
        let is_active = matches!(route, DocsRoute::Home | DocsRoute::CosmoRoot | DocsRoute::Cosmo);

        html!(
            <CosmoMainMenuItemLink<DocsRoute> to={DocsRoute::CosmoRoot} label="Cosmo" is_active={is_active} />
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

fn switch_app(route: DocsRoute) -> Html {
    match route {
        DocsRoute::Home => { html!(<Redirect<DocsRoute> to={DocsRoute::CosmoRoot} />) }
        DocsRoute::CosmoRoot => { html!(<Redirect<CosmoRoute> to={CosmoRoute::About} />) }
        DocsRoute::Cosmo => { html!(<Switch<CosmoRoute> render={switch_cosmo} />) }
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
                        <CosmoTopBarItem label="Topbar item 1" />
                        <CosmoTopBarItem label="Topbar item 2" />
                        <CosmoTopBarItem label="Topbar item 3" />
                    </CosmoTopBar>
                    <CosmoMenuBar>
                        <CosmoMainMenu>
                            <Switch<DocsRoute> render={render_cosmo_main_menu_entry()} />
                        </CosmoMainMenu>
                        <Switch<DocsRoute> render={switch_sub_menu} />
                    </CosmoMenuBar>
                    <CosmoPageBody>
                        <Switch<DocsRoute> render={switch_app} />
                    </CosmoPageBody>
                </CosmoPageLayout>
            </BounceRoot>
        </BrowserRouter>
    )
}
