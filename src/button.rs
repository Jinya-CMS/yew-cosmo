use stylist::Style;
use stylist::yew::{styled_component, use_style};
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
    let back_button_style = use_style!(r#"
grid-column: backbutton;
border: 4px solid var(--control-border-color);
border-radius: 50%;
height: 48px;
width: 48px;
box-sizing: border-box;
background: var(--white);
position: relative;
cursor: pointer;

&::before,
&::after {
    content: '';
    left: 10px;
    position: absolute;
}

&::before {
    border: 4px solid var(--primary-color);
    box-sizing: border-box;
    height: 18px;
    width: 18px;
    display: block;
    border-right: 0;
    border-bottom: 0;
    transform: translateY(-50%) rotate(-45deg);
    position: absolute;
    top: 50%;
}

&::after {
    display: block;
    width: 20px;
    height: 4px;
    background: var(--primary-color);
    top: 18px;
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
    "#);

    #[cfg(feature = "with-yew-router")]
        let navigator = use_navigator();

    #[cfg(feature = "with-yew-router")]
        let disabled_state = use_state_eq(|| navigator.is_none() || BrowserHistory::default().is_empty());

    #[cfg(feature = "with-yew-router")]
        let on_click = use_callback(|_: MouseEvent, (navigator, disabled_state)| {
        if let Some(navigator) = navigator {
            navigator.back();
            disabled_state.set(BrowserHistory::default().is_empty());
        }
    }, (navigator.clone(), disabled_state.clone()));
    #[cfg(not(feature = "with-yew-router"))]
        let on_click = use_callback(|_: MouseEvent, on_click| on_click.emit(()), _props.on_click.clone());

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
    let button_style = use_style!(r#"
cursor: pointer;
font-family: var(--font-family);
font-size: 16px;
padding: 3px 16px;
box-sizing: border-box;
border: 1px solid var(--control-border-color);
background: var(--white);
color: var(--black);
line-height: 19px;
text-decoration: none;
font-weight: normal;

&:disabled {
    cursor: not-allowed;
    border: 1px solid var(--control-border-color);
    background: var(--white);
    color: var(--disabled-color);
}

&:disabled + .cosmo-button,
&:disabled + .cosmo-button:disabled,
+ .cosmo-button {
    border-left-width: 0;
}

&:hover {
    background: var(--primary-color);
    color: var(--white);
    outline: none;
    box-shadow: none;
}

&:disabled:hover {
    background: var(--white);
    color: var(--control-border-color);
    outline: none;
    box-shadow: none;
}

&:focus {
    border-color: var(--primary-color);
    outline: none;
    box-shadow: none;
}
    "#);

    let mut full_width_style: Option<Style> = Some(use_style!(r#"
width: 100%;
text-align: center;
margin-top: auto;
        "#));
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
    let on_click = props.on_click.clone().map(|on_click| Callback::from(move |_| on_click.emit(())));

    html!(
        <button disabled={!props.enabled} type={button_type} onclick={on_click} class={style}>{props.label.clone()}</button>
    )
}

#[cfg(feature = "with-yew-router")]
#[derive(PartialEq, Clone, Properties)]
pub struct CosmoButtonLinkProps<Route> where Route: Routable + 'static {
    pub label: String,
    pub to: Route,
    #[prop_or(false)]
    pub is_full_width: bool,
    #[prop_or(true)]
    pub enabled: bool,
}

#[cfg(feature = "with-yew-router")]
#[function_component(CosmoButtonLink)]
pub fn button_link<Route>(props: &CosmoButtonLinkProps<Route>) -> Html where Route: Routable + 'static {
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
    let button_container_style = use_style!(r#"
display: flex;
justify-content: flex-end;
margin-top: 10px;
gap: 16px;

> .cosmo-button + .cosmo-button
> .cosmo-button:disabled + .cosmo-button,
> .cosmo-button:disabled + .cosmo-button:disabled {
    border-left-width: 1px;
}
    "#);

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
        CosmoCircleButtonSize::Small => "24px",
        CosmoCircleButtonSize::Medium => "32px",
        CosmoCircleButtonSize::Large => "48px",
    };
    let icon_size = match props.size {
        CosmoCircleButtonSize::Small => "16px",
        CosmoCircleButtonSize::Medium => "24px",
        CosmoCircleButtonSize::Large => "32px",
    };
    let button_style = use_style!(r#"
border-radius: 100%;
border: 2px solid var(--control-border-color);
height: ${size};
width: ${size};
background: var(--white);
color: var(--primary-color);
cursor: pointer;
text-decoration: none;
font-weight: normal;
display: inline-flex;
justify-content: center;
align-items: center;

&:disabled {
    cursor: not-allowed;
    background: var(--white);
    color: var(--disabled-color);
}

&:hover {
    background: var(--primary-color);
    color: var(--white);
    outline: none;
    box-shadow: none;
    border-color: var(--primary-color);
}

&:disabled:hover {
    background: var(--white);
    color: var(--control-border-color);
    outline: none;
    box-shadow: none;
}

&:focus {
    border-color: var(--primary-color);
    outline: none;
    box-shadow: none;
}
    "#, size = size);

    let on_click = use_callback(|_: MouseEvent, on_click| if let Some(on_click) = on_click { on_click.emit(()) }, props.on_click.clone());

    html!(
        <button class={button_style} title={props.title.clone()} onclick={on_click}>
            <yew_icons::Icon icon_id={props.icon} width={icon_size} height={icon_size} />
        </button>
    )
}
