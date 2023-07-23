use stylist::Style;
use stylist::yew::{styled_component, use_style};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoCodeProps {
    #[prop_or_default]
    pub children: Children,
}

#[styled_component(CosmoCode)]
pub fn code(props: &CosmoCodeProps) -> Html {
    let style = use_style!(r#"
font-family: "Source Code Pro", monospace;
    "#);

    html!(
        <code class={style}>{for props.children.iter()}</code>
    )
}

#[derive(PartialEq, Clone)]
pub enum CosmoHeaderLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoHeaderProps {
    pub header: AttrValue,
    #[prop_or(CosmoHeaderLevel::H1)]
    pub level: CosmoHeaderLevel,
}

#[styled_component(CosmoHeader)]
pub fn header(props: &CosmoHeaderProps) -> Html {
    let (font_size, tag) = match props.level {
        CosmoHeaderLevel::H1 => ("32px", "h1"),
        CosmoHeaderLevel::H2 => ("28px", "h2"),
        CosmoHeaderLevel::H3 => ("24px", "h3"),
        CosmoHeaderLevel::H4 => ("22px", "h4"),
        CosmoHeaderLevel::H5 => ("20px", "h5"),
        CosmoHeaderLevel::H6 => ("16px", "h6"),
    };

    let style = use_style!(r#"
font-weight: var(--font-weight-light);
font-size: ${font_size};
    "#, font_size = font_size);

    html!(
        <@{tag} class={style}>{props.header.clone()}</@>
    )
}


#[derive(PartialEq, Clone, Properties)]
pub struct CosmoPreProps {
    #[prop_or_default]
    pub children: Children,
}

#[styled_component(CosmoPre)]
pub fn pre(props: &CosmoPreProps) -> Html {
    let style = use_style!(r#"
font-family: "Source Code Pro", monospace;
    "#);

    html!(
        <pre class={style}>{for props.children.iter()}</pre>
    )
}

#[hook]
fn use_anchor_style() -> Style {
    use_style!(r#"
color: var(--primary-color);
    "#)
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoAnchorLinkProps<Route> where Route: Routable + 'static {
    #[prop_or_default]
    pub children: Children,
    pub to: Route,
}

#[styled_component(CosmoAnchorLink)]
pub fn anchor_link<Route>(props: &CosmoAnchorLinkProps<Route>) -> Html where Route: Routable + 'static {
    let style = use_anchor_style();

    html!(
        <Link<Route> to={props.to.clone()} classes={style}>{for props.children.iter()}</Link<Route>>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoAnchorProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub href: AttrValue,
    #[prop_or_default]
    pub on_click: Option<Callback<()>>,
}

#[styled_component(CosmoAnchor)]
pub fn anchor(props: &CosmoAnchorProps) -> Html {
    let style = use_anchor_style();
    let on_click = use_callback(|evt: MouseEvent, callback| {
        if let Some(callback) = callback {
            evt.prevent_default();
            callback.emit(());
        }
    }, props.on_click.clone());

    html!(
        <a href={props.href.clone()} onclick={on_click} class={style}>{for props.children.iter()}</a>
    )
}
