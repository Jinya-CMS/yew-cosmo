use bounce::BounceRoot;
use bounce::helmet::Helmet;
use stylist::{GlobalStyle, Style};
use stylist::yew::{styled_component, use_style};
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::VChild;
#[cfg(feature = "with-yew-router")]
use yew_router::prelude::*;

use crate::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTitleProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub subtitle: Option<AttrValue>,
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

small {
    margin-left: 16px;
}
    "#);

    html!(
        <h1 class={classes!(title_style, "cosmo-title")}>
            {props.title.clone()}
            if let Some(subtitle) = props.subtitle.clone() {
                <small>{subtitle}</small>
            }
        </h1>
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

    let style = GlobalStyle::new(css!(r#"
/* firefox scroll bars */
* {
	scrollbar-width: normal;
	scrollbar-color: var(--primary-color) transparent;
}

/* webkit scrollbars */

::-webkit-scrollbar {
	width: 10px;
}

::-webkit-scrollbar-thumb {
	box-shadow: inset 0 0 0 6px var(--primary-color);
	border: 2px solid transparent;
	border-radius: 5px !important;
}

::-webkit-scrollbar-thumb:window-inactive {
	box-shadow: inset 0 0 0 6px var(--primary-color);
}

::-webkit-scrollbar-thumb:hover {
	box-shadow: inset 0 0 0 6px var(--primary-color);
}

::-webkit-scrollbar-corner {
	background: transparent;
}

::selection {
    background: var(--primary-color);
    color: var(--white);
}

:root {
	--control-border-color: #cccccc;
	--primary-color: ${primary_color};
	--white: #ffffff;
	--black: #333333;
	--menu-text-selected-color: var(--black);
	--menu-text-color: #00000040;
	--disabled-color: var(--menu-text-color);
	--negative-color: #e2180d;
	--negative-light-color: #f8bab8;
	--positive-color: #146621;
	--positive-light-color: #cdecd3;
	--information-color: #182b70;
	--information-light-color: #dde3f8;
	--warning-color: #dfa700;
	--warning-light-color: #ffedb8;
	--code-color: #182b70;
	--gradient-top-color: #ededee;
	--gradient-bottom-color: var(--white);
	--modal-backdrop: #ffffff4d;
	--table-stripe-color: #eeeeee;

	--font-weight-bold: bold;
	--font-weight-normal: normal;
	--font-weight-light: 300;
	--font-family: Lato, sans-serif;

	--dropdown-background: url("data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzMiIgaGVpZ2h0PSI2IiB2aWV3Qm94PSIwIDAgMC4wMDAzMiA2ZS0wNSIgdmVyc2lvbj0iMS4xIj4KICAgIDxnPgogICAgICAgIDxwYXRoIHN0eWxlPSJvcGFjaXR5OjE7ZmlsbDojMDAwMDAwO2ZpbGwtb3BhY2l0eToxO3N0cm9rZS13aWR0aDowLjI2NDk5OTtzdHJva2UtbWl0ZXJsaW1pdDo0O3N0cm9rZS1kYXNoYXJyYXk6bm9uZSIKICAgICAgICAgICAgICBkPSJtIDQuODg5MDY2MSw0LjIzNDA1NTQgLTIuNDQ0NTMzLDFlLTcgTCA3Ljg1NTk1NjZlLTgsNC4yMzQwNTU0IDEuMjIyMjY2NSwyLjExNzAyNzcgMi40NDQ1MzMxLDAgMy42NjY3OTk3LDIuMTE3MDI3NiBaIgogICAgICAgICAgICAgIHRyYW5zZm9ybT0ibWF0cml4KDIuNDU0NDU2NWUtNSwwLDAsLTEuNDE3MDgxMWUtNSw5Ljk5OTk5OThlLTUsNi4wMDAwMDAxZS01KSIvPgogICAgPC9nPgo8L3N2Zz4=");
}

@media screen and (prefers-color-scheme: dark) {
	:root {
		--control-border-color: #333333;
		--primary-color: ${primary_color_dark};
		--white: #000000;
		--black: #cccccc;
		--menu-text-selected-color: var(--black);
		--menu-text-color: #ffffff40;
		--disabled-color: var(--menu-text-color);
		--negative-color: #290403;
		--negative-light-color: #600a06;
		--positive-color: #041406;
		--positive-light-color: #0a3511;
		--information-color: #070c20;
		--information-light-color: #0e1840;
        --warning-color: #7d5e00;
        --warning-light-color: #564000;
		--code-color: #1e368f;
		--gradient-top-color: #121212;
		--gradient-bottom-color: var(--white);
		--modal-backdrop: rgba(0, 0, 0, 0.3);
		--table-stripe-color: #1c1b1b;

	    --dropdown-background: url("data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzMiIgaGVpZ2h0PSI2IiB2aWV3Qm94PSIwIDAgMC4wMDAzMiA2ZS0wNSIgdmVyc2lvbj0iMS4xIj4KICAgIDxnPgogICAgICAgIDxwYXRoIHN0eWxlPSJvcGFjaXR5OjE7ZmlsbDojZmZmZmZmO2ZpbGwtb3BhY2l0eToxO3N0cm9rZS13aWR0aDowLjI2NDk5OTtzdHJva2UtbWl0ZXJsaW1pdDo0O3N0cm9rZS1kYXNoYXJyYXk6bm9uZSIKICAgICAgICAgICAgICBkPSJtIDQuODg5MDY2MSw0LjIzNDA1NTQgLTIuNDQ0NTMzLDFlLTcgTCA3Ljg1NTk1NjZlLTgsNC4yMzQwNTU0IDEuMjIyMjY2NSwyLjExNzAyNzcgMi40NDQ1MzMxLDAgMy42NjY3OTk3LDIuMTE3MDI3NiBaIgogICAgICAgICAgICAgIHRyYW5zZm9ybT0ibWF0cml4KDIuNDU0NDU2NWUtNSwwLDAsLTEuNDE3MDgxMWUtNSw5Ljk5OTk5OThlLTUsNi4wMDAwMDAxZS01KSIvPgogICAgPC9nPgo8L3N2Zz4=");
	}
}

.cosmo--light-theme {
	--control-border-color: #cccccc;
	--primary-color: ${primary_color};
	--white: #ffffff;
	--black: #333333;
	--menu-text-selected-color: var(--black);
	--menu-text-color: #00000040;
	--disabled-color: var(--menu-text-color);
	--negative-color: #e2180d;
	--negative-light-color: #f8bab8;
	--positive-color: #146621;
	--positive-light-color: #cdecd3;
	--information-color: #182b70;
	--information-light-color: #dde3f8;
	--warning-color: #dfa700;
	--warning-light-color: #ffedb8;
	--code-color: #182b70;
	--gradient-top-color: #ededee;
	--gradient-bottom-color: var(--white);
	--modal-backdrop: #ffffff4d;
	--table-stripe-color: #eeeeee;

	--dropdown-background: url("data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzMiIgaGVpZ2h0PSI2IiB2aWV3Qm94PSIwIDAgMC4wMDAzMiA2ZS0wNSIgdmVyc2lvbj0iMS4xIj4KICAgIDxnPgogICAgICAgIDxwYXRoIHN0eWxlPSJvcGFjaXR5OjE7ZmlsbDojMDAwMDAwO2ZpbGwtb3BhY2l0eToxO3N0cm9rZS13aWR0aDowLjI2NDk5OTtzdHJva2UtbWl0ZXJsaW1pdDo0O3N0cm9rZS1kYXNoYXJyYXk6bm9uZSIKICAgICAgICAgICAgICBkPSJtIDQuODg5MDY2MSw0LjIzNDA1NTQgLTIuNDQ0NTMzLDFlLTcgTCA3Ljg1NTk1NjZlLTgsNC4yMzQwNTU0IDEuMjIyMjY2NSwyLjExNzAyNzcgMi40NDQ1MzMxLDAgMy42NjY3OTk3LDIuMTE3MDI3NiBaIgogICAgICAgICAgICAgIHRyYW5zZm9ybT0ibWF0cml4KDIuNDU0NDU2NWUtNSwwLDAsLTEuNDE3MDgxMWUtNSw5Ljk5OTk5OThlLTUsNi4wMDAwMDAxZS01KSIvPgogICAgPC9nPgo8L3N2Zz4=");

	background: var(--white);
	color: var(--black);
}

.cosmo--dark-theme {
	--control-border-color: #333333;
	--primary-color: ${primary_color_dark};
	--white: #000000;
	--black: #cccccc;
	--menu-text-selected-color: var(--black);
	--menu-text-color: #ffffff40;
	--disabled-color: var(--menu-text-color);
	--negative-color: #290403;
	--negative-light-color: #600a06;
	--positive-color: #041406;
	--positive-light-color: #0a3511;
	--information-color: #070c20;
	--information-light-color: #0e1840;
	--warning-color: #7d5e00;
	--warning-light-color: #564000;
	--code-color: #1e368f;
	--gradient-top-color: #121212;
	--gradient-bottom-color: var(--white);
	--modal-backdrop: rgba(0, 0, 0, 0.3);
	--table-stripe-color: #1c1b1b;

    --dropdown-background: url("data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzMiIgaGVpZ2h0PSI2IiB2aWV3Qm94PSIwIDAgMC4wMDAzMiA2ZS0wNSIgdmVyc2lvbj0iMS4xIj4KICAgIDxnPgogICAgICAgIDxwYXRoIHN0eWxlPSJvcGFjaXR5OjE7ZmlsbDojZmZmZmZmO2ZpbGwtb3BhY2l0eToxO3N0cm9rZS13aWR0aDowLjI2NDk5OTtzdHJva2UtbWl0ZXJsaW1pdDo0O3N0cm9rZS1kYXNoYXJyYXk6bm9uZSIKICAgICAgICAgICAgICBkPSJtIDQuODg5MDY2MSw0LjIzNDA1NTQgLTIuNDQ0NTMzLDFlLTcgTCA3Ljg1NTk1NjZlLTgsNC4yMzQwNTU0IDEuMjIyMjY2NSwyLjExNzAyNzcgMi40NDQ1MzMxLDAgMy42NjY3OTk3LDIuMTE3MDI3NiBaIgogICAgICAgICAgICAgIHRyYW5zZm9ybT0ibWF0cml4KDIuNDU0NDU2NWUtNSwwLDAsLTEuNDE3MDgxMWUtNSw5Ljk5OTk5OThlLTUsNi4wMDAwMDAxZS01KSIvPgogICAgPC9nPgo8L3N2Zz4=");

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
    "#,
    primary_color = primary_color,
    primary_color_dark = primary_color_dark,
    )).expect("Should insert global styles");

    let page_layout_style = use_style!(r#"
display: grid;
grid-template-rows: [top-menu] 64px [main-menu] 80px [top-spacing] 32px [content] 1fr [bottom-spacing] 28px [bottom-bar] 68px;
    "#);

    html!(
        <BounceRoot>
            <Helmet>
                <link href="https://fonts.jinya.de/css2?family=Lato:ital,wght@0,100%3B1,100%3B0,300%3B1,300%3B0,400%3B1,400%3B0,700%3B1,700%3B0,900%3B1,900" rel="stylesheet" type="text/css" />
                <link href="https://fonts.jinya.de/css2?family=Source Code Pro" rel="stylesheet" type="text/css" />
                <style>
                    {style.get_style_str()}
                </style>
            </Helmet>
            <div class={page_layout_style}>
                {for props.children.iter()}
            </div>
        </BounceRoot>
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

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTopBarItemProps {
    pub label: AttrValue,
    #[prop_or_default]
    pub on_click: Callback<()>,
}

#[styled_component(CosmoTopBarItem)]
pub fn top_bar_item(props: &CosmoTopBarItemProps) -> Html {
    let style = use_top_bar_item_style();

    let on_click = use_callback(|_: MouseEvent, on_click| on_click.emit(()), props.on_click.clone());

    html!(
        <a class={style} onclick={on_click}>{props.label.clone()}</a>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTopBarItemExternalProps {
    pub label: AttrValue,
    pub href: AttrValue,
}

#[styled_component(CosmoTopBarItemExternal)]
pub fn top_bar_item_external(props: &CosmoTopBarItemExternalProps) -> Html {
    let style = use_top_bar_item_style();

    html!(
        <a class={style} href={props.href.clone()} target="_blank">{props.label.clone()}</a>
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


#[derive(PartialEq, Clone, Properties)]
pub struct CosmoBottomBarItemProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CosmoBottomBarLeftItem)]
pub fn bottom_bar_left_item(props: &CosmoBottomBarItemProps) -> Html {
    html!({for props.children.iter()})
}

#[function_component(CosmoBottomBarRightItem)]
pub fn bottom_bar_right_item(props: &CosmoBottomBarItemProps) -> Html {
    html!({for props.children.iter()})
}

#[derive(Clone, derive_more::From, PartialEq)]
pub enum CosmoBottomBarChildren {
    CosmoBottomBarLeftItem(VChild<CosmoBottomBarLeftItem>),
    CosmoBottomBarRightItem(VChild<CosmoBottomBarRightItem>),
}

#[allow(clippy::from_over_into)]
impl Into<Html> for CosmoBottomBarChildren {
    fn into(self) -> Html {
        match self {
            CosmoBottomBarChildren::CosmoBottomBarLeftItem(child) => child.into(),
            CosmoBottomBarChildren::CosmoBottomBarRightItem(child) => child.into(),
        }
    }
}

impl CosmoBottomBarChildren {
    pub(crate) fn is_left(&self) -> bool {
        matches!(self, CosmoBottomBarChildren::CosmoBottomBarLeftItem(_))
    }

    pub(crate) fn is_right(&self) -> bool {
        matches!(self, CosmoBottomBarChildren::CosmoBottomBarRightItem(_))
    }
}

#[derive(PartialEq, Clone, Default)]
pub enum CosmoBottomBarProgressState {
    #[default]
    Hidden,
    Visible,
    Indeterminate,
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoBottomBarProps {
    #[prop_or_default]
    pub children: ChildrenRenderer<CosmoBottomBarChildren>,
    #[prop_or_default]
    pub progress_top_label: AttrValue,
    #[prop_or_default]
    pub progress_bottom_label: AttrValue,
    #[prop_or_default]
    pub progress_state: CosmoBottomBarProgressState,
    #[prop_or_default]
    pub progress_value: usize,
    #[prop_or_default]
    pub progress_max: usize,
}

#[styled_component(CosmoBottomBar)]
pub fn bottom_bar(props: &CosmoBottomBarProps) -> Html {
    let bottom_bar_style = use_style!(r#"
grid-row: bottom-bar;
align-items: center;
display: grid;
grid-template-columns: [left] 1fr [center] 1fr [right] 1fr;
gap: 1rem;
padding-left: 164px;
padding-right: 164px;
"#);

    let bottom_bar_item_left = use_style!(r#"
grid-column: left;
justify-self: left;
display: flex;
gap: 16px;
align-items: center;
    "#);
    let bottom_bar_item_center = use_style!(r#"
grid-column: center;
justify-self: center;
display: grid;
justify-items: center;
    "#);
    let bottom_bar_item_right = use_style!(r#"
grid-column: right;
justify-self: right;
display: flex;
gap: 16px;
align-items: center;
    "#);
    let progress_bar_top_label = use_style!(r#"
font-size: 16px;
color: var(--black);
display: block;
    "#);
    let progress_bar_bottom_label = use_style!(r#"
font-size: 13px;
color: var(--black);
display: block;
    "#);

    let left = props.children.iter().find(|item| item.is_left());
    let right = props.children.iter().find(|item| item.is_right());

    html!(
        <div class={bottom_bar_style}>
            <div class={bottom_bar_item_left}>
                {left.clone()}
            </div>
            if matches!(props.progress_state, CosmoBottomBarProgressState::Visible | CosmoBottomBarProgressState::Indeterminate) {
                <div class={bottom_bar_item_center}>
                    <span class={progress_bar_top_label}>{props.progress_top_label.clone()}</span>
                    <CosmoProgressBar is_indeterminate={props.progress_state == CosmoBottomBarProgressState::Indeterminate} value={props.progress_value} max={props.progress_max} />
                    <span class={progress_bar_bottom_label}>{props.progress_bottom_label.clone()}</span>
                </div>
            }
            <div class={bottom_bar_item_right}>
                {right.clone()}
            </div>
        </div>
    )
}