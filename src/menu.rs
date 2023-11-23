use stylist::yew::{styled_component, use_style};
use yew::prelude::*;
#[cfg(feature = "with-yew-router")]
use yew_router::prelude::*;

use crate::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoMainMenuProps {
    #[prop_or_default]
    pub children: Children,
}

#[styled_component(CosmoMainMenu)]
pub fn main_menu(props: &CosmoMainMenuProps) -> Html {
    let main_menu_style = use_style!(
        r#"
grid-row: main-menu;
    "#
    );

    html!(
        <div class={main_menu_style}>
            {for props.children.iter()}
        </div>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoMenuBarProps {
    #[prop_or_default]
    pub children: Children,
}

#[styled_component(CosmoMenuBar)]
pub fn menu_bar(props: &CosmoMenuBarProps) -> Html {
    let menu_bar_style = use_style!(
        r#"
grid-row: main-menu;
display: grid;
position: relative;
grid-template-columns:
    [left-touch] var(--menu-left-touch-width) [spacing1] 1rem [backbutton] var(--back-button-width)
    [spacing2] calc(
        var(--page-side-spacing) - var(--menu-left-touch-width) - 1rem - var(--back-button-width)
    )
    [content] 1fr;

&::before {
	content: '';
	position: absolute;
	width: var(--menu-left-touch-width);
	height: 100%;
	background: var(--primary-color);
	border-bottom-right-radius: var(--border-radius);
	border-top-right-radius: var(--border-radius);
}
    "#
    );

    html!(
        <div class={menu_bar_style}>
            <CosmoBackButton />
            <CosmoMenuCollection>
                {for props.children.iter()}
            </CosmoMenuCollection>
        </div>
    )
}

#[derive(PartialEq, Clone, Properties)]
struct CosmoMenuCollectionProps {
    #[prop_or_default]
    pub children: Children,
}

#[styled_component(CosmoMenuCollection)]
fn menu_collection(props: &CosmoMenuCollectionProps) -> Html {
    let menu_collection_style = use_style!(
        r#"
display: grid;
grid-column: content;
grid-template-rows: [main-menu] var(--main-menu-height) [sub-menu] var(--sub-menu-height);
grid-row-gap: var(--menu-gap);
    "#
    );

    html!(
        <nav class={menu_collection_style}>
            {for props.children.iter()}
        </nav>
    )
}

#[cfg(feature = "with-yew-router")]
#[derive(PartialEq, Clone, Properties)]
pub struct CosmoMainMenuItemLinkProps<Route>
where
    Route: Routable + 'static,
{
    pub label: AttrValue,
    pub to: Route,
    pub is_active: bool,
}

#[hook]
fn use_main_menu_item_style(is_active: bool) -> Classes {
    let item_style = use_style!(
        r#"
text-decoration: none;
font-weight: var(--font-weight-menu);
font-family: var(--font-family-menu);
text-transform: lowercase;
font-size: var(--font-size-main-menu);
line-height: var(--font-size-main-menu);
vertical-align: text-top;
color: var(--menu-text-color);
margin-right: calc(var(--font-size-main-menu) / 2);
    "#
    );
    let mut active_style = Some(use_style!(
        r#"
color: var(--menu-text-selected-color);
    "#
    ));
    if !is_active {
        active_style = None;
    }

    classes!(item_style, active_style)
}

#[cfg(feature = "with-yew-router")]
#[styled_component(CosmoMainMenuItemLink)]
pub fn main_menu_item_link<Route>(props: &CosmoMainMenuItemLinkProps<Route>) -> Html
where
    Route: Routable + 'static,
{
    let style = use_main_menu_item_style(props.is_active);

    html!(
        <Link<Route> to={props.to.clone()} classes={style}>{props.label.clone()}</Link<Route>>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoMainMenuItemProps {
    pub label: AttrValue,
    pub is_active: bool,
}

#[styled_component(CosmoMainMenuItem)]
pub fn main_menu_item(props: &CosmoMainMenuItemProps) -> Html {
    let style = use_main_menu_item_style(props.is_active);

    html!(
        <span class={style}>{props.label.clone()}</span>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoSubMenuBarProps {
    #[prop_or_default]
    pub children: Children,
}

#[styled_component(CosmoSubMenuBar)]
pub fn sub_menu_bar(props: &CosmoSubMenuBarProps) -> Html {
    let sub_menu_style = use_style!(
        r#"
grid-row: sub-menu;
    "#
    );

    html!(
        <div class={sub_menu_style}>
            {for props.children.iter()}
        </div>
    )
}

#[hook]
fn use_sub_menu_item_style(is_active: bool) -> Classes {
    let item_style = use_style!(
        r#"
text-decoration: none;
font-weight: var(--font-weight-menu);
font-family: var(--font-family-menu);
text-transform: uppercase;
font-size: var(--font-size-sub-menu);
line-height: var(--font-size-sub-menu);
vertical-align: text-top;
margin-right: var(--font-size-sub-menu);
color: var(--black);
    "#
    );
    let mut active_style = Some(use_style!(
        r#"
font-weight: var(--font-weight-sub-menu-active);
    "#
    ));
    if !is_active {
        active_style = None;
    }

    classes!(item_style, active_style)
}

#[cfg(feature = "with-yew-router")]
#[derive(PartialEq, Clone, Properties)]
pub struct CosmoSubMenuItemLinkProps<Route>
where
    Route: Routable + 'static,
{
    pub label: AttrValue,
    pub to: Route,
    pub is_active: bool,
}

#[cfg(feature = "with-yew-router")]
#[styled_component(CosmoSubMenuItemLink)]
pub fn sub_menu_item_link<Route>(props: &CosmoSubMenuItemLinkProps<Route>) -> Html
where
    Route: Routable + 'static,
{
    let style = use_sub_menu_item_style(props.is_active);

    html!(
        <Link<Route> to={props.to.clone()} classes={style}>{props.label.clone()}</Link<Route>>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoSubMenuItemProps {
    pub label: AttrValue,
    pub is_active: bool,
}

#[styled_component(CosmoSubMenuItem)]
pub fn sub_menu_item(props: &CosmoSubMenuItemProps) -> Html {
    let style = use_sub_menu_item_style(props.is_active);

    html!(
        <span class={style}>{props.label.clone()}</span>
    )
}
