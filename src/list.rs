use stylist::yew::{styled_component, use_style};
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::VChild;

use crate::button::CosmoButton;

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoSideListProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<CosmoSideListItem>,
    #[prop_or(false)]
    pub has_add_button: bool,
    #[prop_or_default]
    pub add_button_label: AttrValue,
    #[prop_or_default]
    pub add_button_on_click: Callback<()>,
}

#[styled_component(CosmoSideList)]
pub fn side_list(props: &CosmoSideListProps) -> Html {
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
    let item_active_style = use_style!(r#"
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
    "#);

    let selected_item_state = use_state_eq(|| if props.children.len() > 0 { Some(0) } else { None });

    html!(
        <div class={list_style}>
            <nav class={list_items_style}>
                {for props.children.iter().enumerate().map(|(idx, child)| {
                    let label = child.props.label.clone();
                    let selected_item_state = selected_item_state.clone();
                    let item_active_style = item_active_style.clone();
                    let item_style = item_style.clone();
                    let on_click = {
                        let selected_item_state = selected_item_state.clone();

                        Callback::from(move |_| {
                            selected_item_state.set(Some(idx));
                        })
                    };
                    let classes = if *selected_item_state == Some(idx) {
                        classes!(item_style, item_active_style)
                    } else {
                        classes!(item_style)
                    };

                    html!(
                        <a class={classes} onclick={on_click}>{label.clone()}</a>
                    )
                })}
                {if props.has_add_button {
                    let on_click = props.add_button_on_click.clone();

                    html!(<CosmoButton is_full_width={true} on_click={move |_| on_click.emit(())} label={props.add_button_label.clone()} />)
                } else {
                    html!()
                }}
            </nav>
            <div class={list_content_style}>
                {if let Some(selected_item) = (*selected_item_state).clone() {
                    if let Some((_, item)) = props.children.iter().enumerate().nth(selected_item).clone() {
                        item.into()
                    } else {
                        html!()
                    }
                } else {
                    html!()
                }}
            </div>
        </div>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoSideListItemProps {
    pub label: AttrValue,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CosmoSideListItem)]
pub fn side_list_item(props: &CosmoSideListItemProps) -> Html {
    html!(
        {for props.children.iter()}
    )
}

impl CosmoSideListItem {
    pub fn new(props: CosmoSideListItemProps) -> VChild<Self> {
        VChild::new(props, None)
    }

    pub fn from_label_and_children(label: AttrValue, children: Html) -> VChild<Self> {
        VChild::new(CosmoSideListItemProps {
            label,
            children: ChildrenRenderer::new(vec![children]),
        }, None)
    }
}
