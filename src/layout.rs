use stylist::{global_style, Style};
use stylist::yew::{styled_component, use_style};
use wasm_bindgen::JsCast;
use web_sys::HtmlLinkElement;
use yew::prelude::*;
#[cfg(feature = "with-yew-router")]
use yew_router::prelude::*;

use crate::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTitleProps {
    pub title: AttrValue,
}

#[styled_component(CosmoTitle)]
pub fn title(props: &CosmoTitleProps) -> Html {
    let title_style = use_style!(r#"
font-weight: var(--font-weight-light);
margin: 0;
vertical-align: text-top;
font-size: 36px;

+ .cosmo-title {
    margin-left: 16px;
    border-left: 1px solid var(--black);
    padding-left: 16px;
    box-sizing: border-box;
}
    "#);

    html!(
        <h1 class={classes!(title_style, "cosmo-title")}>{props.title.clone()}</h1>
    )
}


#[derive(PartialEq, Clone, Properties)]
pub struct CosmoPageBodyProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

#[styled_component(CosmoPageBody)]
pub fn page_body(props: &CosmoPageBodyProps) -> Html {
    let page_body_style = use_style!(r#"
grid-row: content;
height: calc(100vh - 64px - 32px - 80px - 28px - 68px);
display: grid;
grid-template-columns: 164px [content] 1fr 164px;
    "#);
    let page_body_content_style = use_style!(r#"
grid-column: content;
overflow-y: auto;
height: calc(100vh - 64px - 32px - 80px - 28px - 68px);
    "#);

    html!(
        <div class={page_body_style}>
            <div class={classes!(page_body_content_style, props.classes.clone())}>
                {for props.children.iter()}
            </div>
        </div>
    )
}


#[derive(PartialEq, Clone, Properties)]
pub struct CosmoPageLayoutProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(AttrValue::from("#514B57"))]
    pub primary_color: AttrValue,
    #[prop_or(AttrValue::from("#966554"))]
    pub primary_color_dark: AttrValue,
}

#[styled_component(CosmoPageLayout)]
pub fn page_layout(props: &CosmoPageLayoutProps) -> Html {
    let primary_color = props.primary_color.to_string();
    let primary_color_dark = props.primary_color_dark.to_string();

    let link = gloo::utils::document().create_element("link").expect("Link should be creatable").dyn_into::<HtmlLinkElement>().expect("Should convert");
    link.set_href("https://fonts.jinya.de/css2?family=Lato:ital,wght@0,100%3B1,100%3B0,300%3B1,300%3B0,400%3B1,400%3B0,700%3B1,700%3B0,900%3B1,900");
    link.set_rel("stylesheet");
    link.set_type("text/css");

    gloo::utils::head().append_child(&link).expect("Should append link");

    let link = gloo::utils::document().create_element("link").expect("Link should be creatable").dyn_into::<HtmlLinkElement>().expect("Should convert");
    link.set_href("https://fonts.jinya.de/css2?family=Source Code Pro");
    link.set_rel("stylesheet");
    link.set_type("text/css");

    gloo::utils::head().append_child(&link).expect("Should append link");

    global_style!(r#"
:root {
    --control-border-color: #CCCCCC;
    --primary-color: ${primary_color};
    --white: #FFFFFF;
    --black: #333333;
    --menu-text-selected-color: var(--black);
    --menu-text-color: #00000040;
    --disabled-color: var(--menu-text-color);
    --negative-color: #e2180d;
    --positive-color: #146621;
    --code-color: #182B70;
    --gradient-top-color: #EDEDEE;
    --gradient-bottom-color: var(--white);
    --modal-backdrop: #FFFFFF4D;
    --table-stripe-color: #EEEEEE;
    --dropdown-background: url("data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzMiIgaGVpZ2h0PSI2IiB2aWV3Qm94PSIwIDAgMC4wMDAzMiA2ZS0wNSIgdmVyc2lvbj0iMS4xIj4KICAgIDxnPgogICAgICAgIDxwYXRoIHN0eWxlPSJvcGFjaXR5OjE7ZmlsbDojMDAwMDAwO2ZpbGwtb3BhY2l0eToxO3N0cm9rZS13aWR0aDowLjI2NDk5OTtzdHJva2UtbWl0ZXJsaW1pdDo0O3N0cm9rZS1kYXNoYXJyYXk6bm9uZSIKICAgICAgICAgICAgICBkPSJtIDQuODg5MDY2MSw0LjIzNDA1NTQgLTIuNDQ0NTMzLDFlLTcgTCA3Ljg1NTk1NjZlLTgsNC4yMzQwNTU0IDEuMjIyMjY2NSwyLjExNzAyNzcgMi40NDQ1MzMxLDAgMy42NjY3OTk3LDIuMTE3MDI3NiBaIgogICAgICAgICAgICAgIHRyYW5zZm9ybT0ibWF0cml4KDIuNDU0NDU2NWUtNSwwLDAsLTEuNDE3MDgxMWUtNSw5Ljk5OTk5OThlLTUsNi4wMDAwMDAxZS01KSIvPgogICAgPC9nPgo8L3N2Zz4=");

    --font-weight-bold: bold;
    --font-weight-normal: normal;
    --font-weight-light: 300;
    --font-family: Lato, sans-serif;
}

@media screen and (prefers-color-scheme: dark) {
    :root {
        --primary-color: ${primary_color_dark};
        --control-border-color: #333333;
        --white: #000000;
        --black: #cccccc;
        --menu-text-selected-color: var(--black);
        --menu-text-color: #ffffff40;
        --disabled-color: var(--menu-text-color);
        --negative-color: #e2180d;
        --positive-color: #146621;
        --code-color: #1e368f;
        --gradient-top-color: #121212;
        --gradient-bottom-color: var(--white);
        --modal-backdrop: rgba(0, 0, 0, 0.3);
        --table-stripe-color: #1c1b1b;
        --dropdown-background: url("data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzMiIgaGVpZ2h0PSI2IiB2aWV3Qm94PSIwIDAgMC4wMDAzMiA2ZS0wNSIgdmVyc2lvbj0iMS4xIj4KICAgIDxnPgogICAgICAgIDxwYXRoIHN0eWxlPSJvcGFjaXR5OjE7ZmlsbDojY2NjY2NjO2ZpbGwtb3BhY2l0eToxO3N0cm9rZS13aWR0aDowLjI2NDk5OTtzdHJva2UtbWl0ZXJsaW1pdDo0O3N0cm9rZS1kYXNoYXJyYXk6bm9uZSIKICAgICAgICAgICAgICBkPSJtIDQuODg5MDY2MSw0LjIzNDA1NTQgLTIuNDQ0NTMzLDFlLTcgTCA3Ljg1NTk1NjZlLTgsNC4yMzQwNTU0IDEuMjIyMjY2NSwyLjExNzAyNzcgMi40NDQ1MzMxLDAgMy42NjY3OTk3LDIuMTE3MDI3NiBaIgogICAgICAgICAgICAgIHRyYW5zZm9ybT0ibWF0cml4KDIuNDU0NDU2NWUtNSwwLDAsLTEuNDE3MDgxMWUtNSw5Ljk5OTk5OThlLTUsNi4wMDAwMDAxZS01KSIvPgogICAgPC9nPgo8L3N2Zz4=");
    }
}

.cosmo--light-theme {
    --primary-color: ${primary_color};
    --control-border-color: #CCCCCC;
    --white: #FFFFFF;
    --black: #333333;
    --menu-text-selected-color: var(--black);
    --menu-text-color: #00000040;
    --disabled-color: var(--menu-text-color);
    --negative-color: #e2180d;
    --positive-color: #146621;
    --code-color: #182B70;
    --gradient-top-color: #EDEDEE;
    --gradient-bottom-color: var(--white);
    --modal-backdrop: #FFFFFF4D;
    --table-stripe-color: #EEEEEE;
    --dropdown-background: url("data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzMiIgaGVpZ2h0PSI2IiB2aWV3Qm94PSIwIDAgMC4wMDAzMiA2ZS0wNSIgdmVyc2lvbj0iMS4xIj4KICAgIDxnPgogICAgICAgIDxwYXRoIHN0eWxlPSJvcGFjaXR5OjE7ZmlsbDojMDAwMDAwO2ZpbGwtb3BhY2l0eToxO3N0cm9rZS13aWR0aDowLjI2NDk5OTtzdHJva2UtbWl0ZXJsaW1pdDo0O3N0cm9rZS1kYXNoYXJyYXk6bm9uZSIKICAgICAgICAgICAgICBkPSJtIDQuODg5MDY2MSw0LjIzNDA1NTQgLTIuNDQ0NTMzLDFlLTcgTCA3Ljg1NTk1NjZlLTgsNC4yMzQwNTU0IDEuMjIyMjY2NSwyLjExNzAyNzcgMi40NDQ1MzMxLDAgMy42NjY3OTk3LDIuMTE3MDI3NiBaIgogICAgICAgICAgICAgIHRyYW5zZm9ybT0ibWF0cml4KDIuNDU0NDU2NWUtNSwwLDAsLTEuNDE3MDgxMWUtNSw5Ljk5OTk5OThlLTUsNi4wMDAwMDAxZS01KSIvPgogICAgPC9nPgo8L3N2Zz4=");

    background: var(--white);
    color: var(--black);
}

.cosmo--dark-theme {
    --primary-color: ${primary_color_dark};
    --control-border-color: #333333;
    --white: #000000;
    --black: #cccccc;
    --menu-text-selected-color: var(--black);
    --menu-text-color: #ffffff40;
    --disabled-color: var(--menu-text-color);
    --negative-color: #e2180d;
    --positive-color: #146621;
    --code-color: #1e368f;
    --gradient-top-color: #121212;
    --gradient-bottom-color: var(--white);
    --modal-backdrop: rgba(0, 0, 0, 0.3);
    --table-stripe-color: #1c1b1b;
    --dropdown-background: url("data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzMiIgaGVpZ2h0PSI2IiB2aWV3Qm94PSIwIDAgMC4wMDAzMiA2ZS0wNSIgdmVyc2lvbj0iMS4xIj4KICAgIDxnPgogICAgICAgIDxwYXRoIHN0eWxlPSJvcGFjaXR5OjE7ZmlsbDojY2NjY2NjO2ZpbGwtb3BhY2l0eToxO3N0cm9rZS13aWR0aDowLjI2NDk5OTtzdHJva2UtbWl0ZXJsaW1pdDo0O3N0cm9rZS1kYXNoYXJyYXk6bm9uZSIKICAgICAgICAgICAgICBkPSJtIDQuODg5MDY2MSw0LjIzNDA1NTQgLTIuNDQ0NTMzLDFlLTcgTCA3Ljg1NTk1NjZlLTgsNC4yMzQwNTU0IDEuMjIyMjY2NSwyLjExNzAyNzcgMi40NDQ1MzMxLDAgMy42NjY3OTk3LDIuMTE3MDI3NiBaIgogICAgICAgICAgICAgIHRyYW5zZm9ybT0ibWF0cml4KDIuNDU0NDU2NWUtNSwwLDAsLTEuNDE3MDgxMWUtNSw5Ljk5OTk5OThlLTUsNi4wMDAwMDAxZS01KSIvPgogICAgPC9nPgo8L3N2Zz4=");
    background: var(--white);
    color: var(--black);
}

body {
    font-family: var(--font-family);
    margin: 0;
    padding: 0;
    background: var(--white);
    color: var(--black);
}
    "#, primary_color = primary_color, primary_color_dark = primary_color_dark).expect("Should insert global styles");

    let page_layout_style = use_style!(r#"
display: grid;
grid-template-rows: [top-menu] 64px [main-menu] 80px [top-spacing] 32px [content] 1fr [bottom-spacing] 28px [bottom-bar] 68px;
    "#);

    html!(
        <div class={page_layout_style}>
            {for props.children.iter()}
        </div>
    )
}


#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTopBarProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(false)]
    pub has_right_item: bool,
    #[prop_or_default]
    pub right_item_label: AttrValue,
    #[prop_or_default]
    pub right_item_on_click: Callback<()>,
}

#[styled_component(CosmoTopBar)]
pub fn top_bar(props: &CosmoTopBarProps) -> Html {
    let top_bar_style = use_style!(r#"
display: grid;
grid-template-columns: [left-column] 164px [content] 1fr [profilepicture] 64px [right-column] 164px;
grid-row: top-menu;
background: linear-gradient(to top, var(--gradient-bottom-color) 0%, var(--gradient-top-color) 100%);
    "#);

    let top_bar_item_style = use_top_bar_item_style();
    let top_bar_item_right_style = use_style!(r#"
margin-left: 16px;
    "#);
    let profile_picture_style = use_style!(r#"
display: block;
width: 64px;
height: 64px;
background: var(--primary-color);
grid-column: profilepicture;
object-fit: cover;
    "#);

    let on_click = props.right_item_on_click.clone();
    html!(
        <div class={top_bar_style}>
            <CosmoTopBarMenu>
                {for props.children.iter()}
            </CosmoTopBarMenu>
            <div class={profile_picture_style}></div>
            if props.has_right_item {
                <a class={classes!(top_bar_item_style, top_bar_item_right_style)} onclick={move |_| on_click.emit(())}>{props.right_item_label.clone()}</a>
            }
        </div>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTopBarItemProps {
    pub label: AttrValue,
    #[prop_or_default]
    pub on_click: Callback<()>,
}

#[hook]
fn use_top_bar_item_style() -> Style {
    use_style!(r#"
display: flex;
height: 100%;
align-items: center;
padding: 0;
text-decoration: none;
text-transform: uppercase;
font-weight: var(--font-weight-light);
color: var(--black);
cursor: pointer;
margin-right: 16px;

&:after {
    content: '';
    height: 16px;
    width: 1px;
    background: var(--black);
    margin-top: 4px;
    margin-left: 16px;
}

&:last-child:after {
    content: unset;
}
    "#)
}

#[styled_component(CosmoTopBarItem)]
pub fn top_bar_item(props: &CosmoTopBarItemProps) -> Html {
    let style = use_top_bar_item_style();

    let on_click = use_callback(|_: MouseEvent, on_click| on_click.emit(()), props.on_click.clone());

    html!(
        <a class={style} onclick={on_click}>{props.label.clone()}</a>
    )
}

#[cfg(feature = "with-yew-router")]
#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTopBarItemLinkProps<Route> where Route: Routable + 'static {
    pub label: AttrValue,
    pub to: Route,
}

#[cfg(feature = "with-yew-router")]
#[styled_component(CosmoTopBarItemLink)]
pub fn top_bar_item<Route>(props: &CosmoTopBarItemLinkProps<Route>) -> Html where Route: Routable + 'static {
    let style = use_top_bar_item_style();

    html!(
        <Link<Route> to={props.to.clone()} classes={style}>{props.label.clone()}</Link<Route>>
    )
}