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
    let main_menu_style = use_style!(r#"
grid-row: main-menu;
    "#);

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
    let menu_bar_style = use_style!(r#"
grid-row: main-menu;
display: grid;
grid-template-columns: [spacing1] 18px [backbutton] 48px [spacing2] 74px [content] 1fr;
border-left: 24px solid var(--primary-color);
    "#);

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
    let menu_collection_style = use_style!(r#"
display: grid;
grid-column: content;
grid-template-rows: [main-menu] 48px [sub-menu] 16px;
grid-row-gap: 16px;
    "#);

    html!(
        <nav class={menu_collection_style}>
            {for props.children.iter()}
        </nav>
    )
}


#[cfg(feature = "with-yew-router")]
#[derive(PartialEq, Clone, Properties)]
pub struct CosmoMainMenuItemLinkProps<Route> where Route: Routable + 'static {
    pub label: AttrValue,
    pub to: Route,
    pub is_active: bool,
}

#[hook]
fn use_main_menu_item_style(is_active: bool) -> Classes {
    let item_style = use_style!(r#"
text-transform: lowercase;
font-size: 48px;
font-weight: var(--font-weight-light);
line-height: 48px;
vertical-align: text-top;
color: var(--menu-text-color);
text-decoration: none;
margin-right: 24px;
    "#);
    let mut active_style = Some(use_style!(r#"
color: var(--menu-text-selected-color);
    "#));
    if !is_active {
        active_style = None;
    }

    classes!(item_style, active_style)
}

#[cfg(feature = "with-yew-router")]
#[styled_component(CosmoMainMenuItemLink)]
pub fn main_menu_item_link<Route>(props: &CosmoMainMenuItemLinkProps<Route>) -> Html where Route: Routable + 'static {
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
    let sub_menu_style = use_style!(r#"
grid-row: sub-menu;
    "#);

    html!(
        <div class={sub_menu_style}>
            {for props.children.iter()}
        </div>
    )
}

#[hook]
fn use_sub_menu_item_style(is_active: bool) -> Classes {
    let item_style = use_style!(r#"
text-transform: uppercase;
font-size: 16px;
font-weight: var(--font-weight-light);
line-height: 16px;
vertical-align: text-top;
margin-right: 16px;
text-decoration: none;
color: var(--black);
    "#);
    let mut active_style = Some(use_style!(r#"
font-weight: var(--font-weight-bold);
    "#));
    if !is_active {
        active_style = None;
    }

    classes!(item_style, active_style)
}

#[cfg(feature = "with-yew-router")]
#[derive(PartialEq, Clone, Properties)]
pub struct CosmoSubMenuItemLinkProps<Route> where Route: Routable + 'static {
    pub label: AttrValue,
    pub to: Route,
    pub is_active: bool,
}

#[cfg(feature = "with-yew-router")]
#[styled_component(CosmoSubMenuItemLink)]
pub fn sub_menu_item_link<Route>(props: &CosmoSubMenuItemLinkProps<Route>) -> Html where Route: Routable + 'static {
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


#[derive(PartialEq, Clone, Properties)]
pub(crate) struct CosmoTopBarMenuProps {
    #[prop_or_default]
    pub children: Children,
}

#[styled_component(CosmoTopBarMenu)]
pub(crate) fn top_bar_menu(props: &CosmoTopBarMenuProps) -> Html {
    let menu_style = use_style!(r#"
display: flex;
justify-content: flex-end;
flex-flow: row nowrap;
grid-column: content;
    "#);

    html!(
        <div class={menu_style}>
            {for props.children.iter()}
        </div>
    )
}