use stylist::yew::{styled_component, use_style};
use stylist::Style;
use yew::prelude::*;
#[cfg(feature = "with-yew-router")]
use yew_router::history::{BrowserHistory, History};
#[cfg(feature = "with-yew-router")]
use yew_router::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoBackButtonProps {
    #[cfg(not(feature = "with-yew-router"))]
    pub on_click: Callback<()>,
    #[cfg(not(feature = "with-yew-router"))]
    #[prop_or_default]
    pub is_enabled: bool,
}

#[styled_component(CosmoBackButton)]
pub fn back_button(_props: &CosmoBackButtonProps) -> Html {
    let back_button_style = use_style!(
        r#"
grid-column: backbutton;
border: var(--back-button-border-width) solid var(--control-border-color);
border-radius: 50%;
height: var(--back-button-width);
width: var(--back-button-width);
box-sizing: border-box;
background: var(--white);
position: relative;
cursor: pointer;

&::before,
&::after {
	content: '';
	position: absolute;
	display: block;
	top: 50%;
	left: 50%;
}

&::before {
	border: var(--back-button-arrow-stroke-width) solid var(--primary-color);
	box-sizing: border-box;
	height: var(--back-button-arrow-fin-width);
	width: var(--back-button-arrow-fin-width);
	border-right: 0;
	border-bottom: 0;
	transform: translateY(-50%) translateX(-50%) rotate(-45deg);
}

&::after {
	width: var(--back-button-arrow-width);
	height: var(--back-button-arrow-stroke-width);
	background: var(--primary-color);
	transform: translateY(-50%) translateX(-50%);
}

&:hover {
    border-color: var(--primary-color);
}

&:disabled {
    border-color: var(--control-border-color);
    cursor: not-allowed;
}

&:disabled::after {
    background: var(--control-border-color);
}

&:disabled::before {
    border-color: var(--control-border-color);
}
    "#
    );

    #[cfg(feature = "with-yew-router")]
    let navigator = use_navigator();

    #[cfg(feature = "with-yew-router")]
    let disabled_state =
        use_state_eq(|| navigator.is_none() || BrowserHistory::default().is_empty());

    #[cfg(feature = "with-yew-router")]
    let on_click = use_callback((navigator.clone(), disabled_state.clone()),|_: MouseEvent, (navigator, disabled_state)| {
        if let Some(navigator) = navigator {
            navigator.back();
            disabled_state.set(BrowserHistory::default().is_empty());
        }
    });
    #[cfg(not(feature = "with-yew-router"))]
    let on_click = use_callback(_props.on_click.clone(), |_: MouseEvent, on_click| on_click.emit(()));

    #[cfg(feature = "with-yew-router")]
    return html!(
        <button onclick={on_click} class={back_button_style} type="button" disabled={*disabled_state}></button>
    );
    #[cfg(not(feature = "with-yew-router"))]
    return html!(
        <button onclick={on_click} class={back_button_style} type="button" disabled={!_props.is_enabled}></button>
    );
}

#[hook]
fn use_cosmo_button_style(is_full_width: bool) -> Classes {
    let button_style = use_style!(
        r#"
cursor: pointer;
font-family: var(--font-family);
font-size: var(--font-size);
padding: var(--button-padding-top) var(--button-padding-right) var(--button-padding-bottom)
    var(--button-padding-left);
box-sizing: border-box;
border: var(--button-border-width) solid var(--button-border-color);
background: var(--button-background);
color: var(--button-color);
line-height: var(--line-height);
height: var(--control-height);
text-decoration: none;
font-weight: var(--font-weight-normal);
border-radius: var(--border-radius);
display: inline-flex;
justify-content: center;
align-items: center;
white-space: nowrap;
transition:
    background-color var(--transition-duration),
    color var(--transition-duration),
    border-color var(--transition-duration);

&:disabled {
    cursor: not-allowed;
    filter: var(--button-disabled-filter);
    outline: none;
    box-shadow: none;
}

&:not(:disabled):hover,
&:not(:disabled):focus {
	--button-background: var(--control-border-color-dark);
	--button-border-color: var(--control-border-color-dark);

	outline: none;
	box-shadow: none;
}

&:not(:disabled):active {
	--button-border-color: var(--control-border-color-darker);
	--button-background: var(--control-border-color-darker);
}
    "#
    );

    let mut full_width_style: Option<Style> = Some(use_style!(
        r#"
width: 100%;
text-align: center;
        "#
    ));
    if !is_full_width {
        full_width_style = None;
    }

    classes!(button_style, full_width_style, "cosmo-button")
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoButtonProps {
    pub label: AttrValue,
    #[prop_or_default]
    pub on_click: Option<Callback<()>>,
    #[prop_or(false)]
    pub is_submit: bool,
    #[prop_or(false)]
    pub is_full_width: bool,
    #[prop_or(true)]
    pub enabled: bool,
}

#[function_component(CosmoButton)]
pub fn button(props: &CosmoButtonProps) -> Html {
    let style = use_cosmo_button_style(props.is_full_width);

    let button_type = if props.is_submit { "submit" } else { "button" };
    let on_click = props
        .on_click
        .clone()
        .map(|on_click| Callback::from(move |_| on_click.emit(())));

    html!(
        <button disabled={!props.enabled} type={button_type} onclick={on_click} class={style}>{props.label.clone()}</button>
    )
}

#[cfg(feature = "with-yew-router")]
#[derive(PartialEq, Clone, Properties)]
pub struct CosmoButtonLinkProps<Route>
where
    Route: Routable + 'static,
{
    pub label: String,
    pub to: Route,
    #[prop_or(false)]
    pub is_full_width: bool,
    #[prop_or(true)]
    pub enabled: bool,
}

#[cfg(feature = "with-yew-router")]
#[function_component(CosmoButtonLink)]
pub fn button_link<Route>(props: &CosmoButtonLinkProps<Route>) -> Html
where
    Route: Routable + 'static,
{
    let style = use_cosmo_button_style(props.is_full_width);

    html!(
        <Link<Route> disabled={!props.enabled} to={props.to.clone()} classes={style}>{props.label.clone()}</Link<Route>>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoButtonContainerProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CosmoButtonContainer)]
pub fn button_container(props: &CosmoButtonContainerProps) -> Html {
    let button_container_style = use_style!(
        r#"
	display: flex;
	justify-content: flex-end;
	margin-top: var(--button-container-margin-top);
	gap: var(--button-container-gap);
    "#
    );

    html!(
        <div class={button_container_style}>
            {for props.children.iter()}
        </div>
    )
}

#[cfg(feature = "with-icons")]
#[derive(PartialEq, Clone)]
pub enum CosmoCircleButtonSize {
    Small,
    Medium,
    Large,
}

#[cfg(feature = "with-icons")]
#[derive(PartialEq, Clone, Properties)]
pub struct CosmoCircleButtonProps {
    pub icon: yew_icons::IconId,
    pub title: AttrValue,
    #[prop_or_default]
    pub on_click: Option<Callback<()>>,
    #[prop_or(true)]
    pub enabled: bool,
    #[prop_or(CosmoCircleButtonSize::Medium)]
    pub size: CosmoCircleButtonSize,
}

#[cfg(feature = "with-icons")]
#[function_component(CosmoCircleButton)]
pub fn circle_button(props: &CosmoCircleButtonProps) -> Html {
    let size = match props.size {
        CosmoCircleButtonSize::Small => "var(--button-circle-size-small)",
        CosmoCircleButtonSize::Medium => "var(--button-circle-size-regular)",
        CosmoCircleButtonSize::Large => "var(--button-circle-size-large)",
    };
    let button_style = use_cosmo_button_style(false);
    let circle_style = use_style!(
        r#"
--border-radius: calc(var(--size) / 2);
--size: ${size};
--button-border-color: var(--button-color);
--button-border-width: var(--button-circle-border-width);

height: var(--size);
min-width: var(--size);
padding: var(--button-circle-padding);
background: var(--button-circle-background);
color: var(--button-border-color);
    "#,
        size = size
    );

    let on_click = use_callback(props.on_click.clone(),|_: MouseEvent, on_click| {
        if let Some(on_click) = on_click {
            on_click.emit(())
        }
    });

    html!(
        <button class={classes!(button_style, circle_style)} title={props.title.clone()} onclick={on_click}>
            <yew_icons::Icon icon_id={props.icon} width="auto" height="auto" />
        </button>
    )
}
