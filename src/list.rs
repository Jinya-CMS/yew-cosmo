use stylist::yew::{styled_component, use_style};
use yew::prelude::*;
#[cfg(feature = "with-yew-router")]
use yew_router::prelude::*;

use crate::button::CosmoButton;

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoMasterDetailListProps {
    #[prop_or_default]
    pub children: Children,
    pub items: Children,
    #[prop_or(false)]
    pub has_add_button: bool,
    #[prop_or_default]
    pub add_button_label: AttrValue,
    #[prop_or_default]
    pub add_button_on_click: Callback<()>,
}

#[styled_component(CosmoMasterDetailList)]
pub fn master_detail_list(props: &CosmoMasterDetailListProps) -> Html {
    let list_style = use_style!(r#"
display: grid;
grid-template-columns: [items-list] 212px 16px [content] 1fr;
height: calc(100vh - 64px - 80px - 32px - 28px - 68px);
    "#);
    let list_items_style = use_style!(r#"
grid-column: items-list;
display: flex;
flex-flow: column;
padding-right: 16px;
border-right: 1px solid var(--control-border-color);
box-sizing: border-box;
height: calc(100vh - 64px - 80px - 32px - 28px - 68px);
overflow-y: auto;
"#);
    let list_content_style = use_style!(r#"
grid-column: content;
height: 100%;
overflow-y: auto;
    "#);

    html!(
        <div class={list_style}>
            <nav class={list_items_style}>
                {for props.items.iter()}
                {if props.has_add_button {
                    let on_click = props.add_button_on_click.clone();

                    html!(<CosmoButton is_full_width={true} on_click={move |_| on_click.emit(())} label={props.add_button_label.clone()} />)
                } else {
                    html!()
                }}
            </nav>
            <div class={list_content_style}>
                {for props.children.iter()}
            </div>
        </div>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoMasterDetailListItemProps {
    pub label: AttrValue,
    #[prop_or_default]
    pub on_click: Option<Callback<()>>,
    #[prop_or(false)]
    pub is_active: bool,
}

#[hook]
fn use_master_detail_list_item_style(is_active: bool) -> Classes {
    let item_style = use_style!(r#"
color: var(--black);
padding: 4px 8px;
overflow-x: hidden;
white-space: pre;
text-overflow: ellipsis;
width: 100%;
box-sizing: border-box;
cursor: pointer;
min-height: 29px;
text-decoration: none;

&:hover {
    background: var(--control-border-color);
}
    "#);
    let mut item_active_style = Some(use_style!(r#"
background: var(--primary-color);
color: var(--white);
font-weight: var(--font-weight-bold);

&:hover {
    background: var(--primary-color);
    color: var(--white);
    font-weight: var(--font-weight-bold);
}

&::selection,
&:hover::selection {
    background: var(--white);
    color: var(--primary-color);
}
    "#));
    if !is_active {
        item_active_style = None;
    }

    classes!(item_style, item_active_style)
}

#[styled_component(CosmoMasterDetailListItem)]
pub fn master_detail_list_item(props: &CosmoMasterDetailListItemProps) -> Html {
    let on_click = props.on_click.clone().map(|on_click| Callback::from(move |_| on_click.emit(())));
    let style = use_master_detail_list_item_style(props.is_active);

    html!(
        <a class={style} onclick={on_click}>{props.label.clone()}</a>
    )
}

#[cfg(feature = "with-yew-router")]
#[derive(PartialEq, Clone, Properties)]
pub struct CosmoMasterDetailListItemLinkProps<Route> where Route: Routable + 'static {
    pub label: AttrValue,
    pub to: Route,
    #[prop_or(false)]
    pub is_active: bool,
}

#[cfg(feature = "with-yew-router")]
#[styled_component(CosmoMasterDetailListItemLink)]
pub fn master_detail_list_item_link<Route>(props: &CosmoMasterDetailListItemLinkProps<Route>) -> Html where Route: Routable + 'static {
    let style = use_master_detail_list_item_style(props.is_active);

    html!(
        <Link<Route> to={props.to.clone()} classes={style}>{props.label.clone()}</Link<Route>>
    )
}
