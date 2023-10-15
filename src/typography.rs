use stylist::yew::{styled_component, use_style};
use stylist::Style;
use yew::prelude::*;
#[cfg(feature = "with-yew-router")]
use yew_router::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoCodeProps {
    #[prop_or_default]
    pub children: Children,
}

#[styled_component(CosmoCode)]
pub fn code(props: &CosmoCodeProps) -> Html {
    let style = use_style!(
        r#"
font-family: "Source Code Pro", monospace;
    "#
    );

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

    let style = use_style!(
        r#"
font-weight: var(--font-weight-light);
font-size: ${font_size};
    "#,
        font_size = font_size
    );

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
    let style = use_style!(
        r#"
font-family: "Source Code Pro", monospace;
    "#
    );

    html!(
        <pre class={style}>{for props.children.iter()}</pre>
    )
}

#[hook]
fn use_anchor_style() -> Style {
    use_style!(
        r#"
color: var(--primary-color);
    "#
    )
}

#[cfg(feature = "with-yew-router")]
#[derive(PartialEq, Clone, Properties)]
pub struct CosmoAnchorLinkProps<Route>
where
    Route: Routable + 'static,
{
    #[prop_or_default]
    pub children: Children,
    pub to: Route,
}

#[cfg(feature = "with-yew-router")]
#[styled_component(CosmoAnchorLink)]
pub fn anchor_link<Route>(props: &CosmoAnchorLinkProps<Route>) -> Html
where
    Route: Routable + 'static,
{
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
    let on_click = use_callback(props.on_click.clone(),|evt: MouseEvent, callback| {
        if let Some(callback) = callback {
            evt.prevent_default();
            callback.emit(());
        }
    });

    html!(
        <a href={props.href.clone()} onclick={on_click} class={style}>{for props.children.iter()}</a>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoParagraphProps {
    #[prop_or_default]
    pub children: Children,
}

#[styled_component(CosmoParagraph)]
pub fn paragraph(props: &CosmoParagraphProps) -> Html {
    let style = use_style!(
        r#"
font-family: var(--font-family);
font-size: 16px;
    "#
    );

    html!(
        <p class={style}>{for props.children.iter()}</p>
    )
}

#[styled_component(CosmoHr)]
pub fn hr() -> Html {
    let style = use_style!(
        r#"
background: radial-gradient(circle, var(--primary-color) 0%, var(--white) 100%);
height: 2px;
border: 0;
margin: 32px 0;
    "#
    );

    html!(
        <hr class={style} />
    )
}

#[styled_component(CosmoBr)]
pub fn br() -> Html {
    html!(
        <br />
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoStrongProps {
    #[prop_or_default]
    pub children: Children,
}

#[styled_component(CosmoStrong)]
pub fn strong(props: &CosmoStrongProps) -> Html {
    html!(
        <strong>{for props.children.iter()}</strong>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoEmProps {
    #[prop_or_default]
    pub children: Children,
}

#[styled_component(CosmoEm)]
pub fn strong(props: &CosmoEmProps) -> Html {
    html!(
        <em>{for props.children.iter()}</em>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoKeyValueListProps {
    #[prop_or_default]
    pub children: Children,
}

#[styled_component(CosmoKeyValueList)]
pub fn key_value_list(props: &CosmoKeyValueListProps) -> Html {
    let key_value_list_style = use_style!(
        r#"
margin: 0;
padding: 0;
display: grid;
    "#
    );

    html!(
        <dl class={key_value_list_style}>
            {for props.children.iter()}
        </dl>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoKeyValueListItemProps {
    #[prop_or_default]
    pub children: Children,
    pub title: AttrValue,
}

#[styled_component(CosmoKeyValueListItem)]
pub fn key_value_list(props: &CosmoKeyValueListItemProps) -> Html {
    let key_value_list_key = use_style!(
        r#"
font-weight: var(--font-weight-normal);
margin: 0;
padding: 0;
    "#
    );
    let key_value_list_value = use_style!(
        r#"
font-weight: var(--font-weight-light);
padding: 0;
margin: 0 0 8px;
    "#
    );

    html!(
        <>
            <dt class={key_value_list_key}>{props.title.clone()}</dt>
            <dd class={key_value_list_value}>
                {for props.children.iter()}
            </dd>
        </>
    )
}
