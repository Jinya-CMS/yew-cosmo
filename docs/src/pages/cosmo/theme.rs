use bounce::helmet::Helmet;
use yew::prelude::*;

use yew_cosmo::prelude::*;

#[function_component(Theme)]
pub fn theme() -> Html {
    let light_theme_alert_state = use_state_eq(|| false);
    let dark_theme_alert_state = use_state_eq(|| false);

    let theme_state = use_state_eq(|| CosmoTheme::Auto);

    let on_open_light_theme_dialog = use_callback(
        |_: (), state| state.set(true),
        light_theme_alert_state.clone(),
    );
    let on_open_dark_theme_dialog = use_callback(
        |_: (), state| state.set(true),
        dark_theme_alert_state.clone(),
    );
    let on_toggle_theme = use_callback(
        |value: Option<AttrValue>, state| match value {
            None => state.set(CosmoTheme::Auto),
            Some(val) => {
                if val == "dark" {
                    state.set(CosmoTheme::Dark)
                } else {
                    state.set(CosmoTheme::Light)
                }
            }
        },
        theme_state.clone(),
    );

    html!(
        <>
            <Helmet>
                <body class={(*theme_state).clone()} />
            </Helmet>
            <CosmoTitle title="Supported themes" />
            <CosmoParagraph>
                {"Since a few versions, browser support an automatic switch between dark and light theme. This switch is based on the theme setting in your operating system. There might be situations, where you want to force light or dark theme. For that purpose Cosmo supports a forced theme. Check below to see which classes you need to apply."}
            </CosmoParagraph>
            <CosmoHeader level={CosmoHeaderLevel::H2} header="Force a theme" />
            <CosmoParagraph>
                {"To force the light theme apply the "}<CosmoCode>{"cosmo--light-theme"}</CosmoCode>{" to the HTML tag of your document. On the other hand, you need to apply the "}<CosmoCode>{"cosmo--dark-theme"}</CosmoCode>{" class to the HTML tag to force the dark theme. "}
            </CosmoParagraph>
            <CosmoParagraph>
                <CosmoDropdown value={match (*theme_state).clone() {
                    CosmoTheme::Auto => None,
                    CosmoTheme::Dark => Some("dark"),
                    CosmoTheme::Light => Some("light"),
                }} label="Toggle theme" on_select={on_toggle_theme} items={vec![(None, AttrValue::from("Automatic")), (Some(AttrValue::from("light")), AttrValue::from("Light theme")), (Some(AttrValue::from("dark")), AttrValue::from("Dark theme"))]} />
            </CosmoParagraph>
            <CosmoHeader level={CosmoHeaderLevel::H2} header="Apply theme only to dialogs" />
            <CosmoParagraph>
                {"It is also possible to force a theme on modal dialogs, by applying setting the theme prop to either Light or Dark."}
            </CosmoParagraph>
            <CosmoParagraph>
                <CosmoButton label="Open light theme dialog" on_click={on_open_light_theme_dialog} />
                <CosmoButton label="Open dark theme dialog" on_click={on_open_dark_theme_dialog} />
            </CosmoParagraph>
            if *light_theme_alert_state {
                <CosmoAlert theme={CosmoTheme::Light} title="Light theme" message="I am a light theme dialog" close_label="Close" on_close={move |_| light_theme_alert_state.set(false)} />
            }
            if *dark_theme_alert_state {
                <CosmoAlert theme={CosmoTheme::Dark} title="Dark theme" message="I am a dark theme dialog" close_label="Close" on_close={move |_| dark_theme_alert_state.set(false)} />
            }
        </>
    )
}
