use stylist::Style;
use stylist::yew::{styled_component, use_style};
use yew::prelude::*;
use yew_router::history::{BrowserHistory, History};
use yew_router::prelude::*;

#[styled_component(CosmoBackButton)]
pub fn back_button() -> Html {
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

    let navigator = use_navigator();

    let disabled_state = use_state_eq(|| navigator.is_none() || BrowserHistory::default().is_empty());

    let on_click = use_callback(|_: MouseEvent, (navigator, disabled_state)| {
        if let Some(navigator) = navigator {
            navigator.back();
            disabled_state.set(BrowserHistory::default().is_empty());
        }
    }, (navigator.clone(), disabled_state.clone()));

    html!(
        <button onclick={on_click} class={back_button_style} type="button" disabled={*disabled_state}></button>
    )
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
.cosmo-button + .cosmo-button {
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

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoButtonLinkProps<Route> where Route: Routable + 'static {
    pub label: String,
    pub to: Route,
    #[prop_or(false)]
    pub is_full_width: bool,
    #[prop_or(true)]
    pub enabled: bool,
}

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

> .cosmo-button + .cosmo-button {
    border-left-width: 1px;
}
    "#);

    html!(
        <div class={button_container_style}>
            {for props.children.iter()}
        </div>
    )
}