use stylist::yew::{styled_component, use_style};
use yew::prelude::*;
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

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoMasterDetailListItemLinkProps<Route> where Route: Routable + 'static {
    pub label: AttrValue,
    pub to: Route,
    #[prop_or(false)]
    pub is_active: bool,
}

#[styled_component(CosmoMasterDetailListItemLink)]
pub fn master_detail_list_item_link<Route>(props: &CosmoMasterDetailListItemLinkProps<Route>) -> Html where Route: Routable + 'static {
    let style = use_master_detail_list_item_style(props.is_active);

    html!(
        <Link<Route> to={props.to.clone()} classes={style}>{props.label.clone()}</Link<Route>>
    )
}


#[derive(PartialEq, Clone, Properties)]
pub struct CosmoKeyValueListProps {
    #[prop_or_default]
    pub children: Children,
}

#[styled_component(CosmoKeyValueList)]
pub fn key_value_list(props: &CosmoKeyValueListProps) -> Html {
    let key_value_list_style = use_style!(r#"
margin: 0;
padding: 0;
display: grid;
    "#);

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
    let key_value_list_key = use_style!(r#"
font-weight: var(--font-weight-normal);
margin: 0;
padding: 0;
    "#);
    let key_value_list_value = use_style!(r#"
font-weight: var(--font-weight-light);
padding: 0;
margin: 0 0 8px;
    "#);

    html!(
        <>
            <dt class={key_value_list_key}>{props.title.clone()}</dt>
            <dd class={key_value_list_value}>
                {for props.children.iter()}
            </dd>
        </>
    )
}