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
    #[prop_or_default]
    pub selected_index: Option<usize>,
    #[prop_or_default]
    pub on_select_item: Option<Callback<usize>>,
}

#[styled_component(CosmoSideList)]
pub fn side_list(props: &CosmoSideListProps) -> Html {
    let list_style = use_style!(
        r#"
display: grid;
grid-template-columns: [items-list] var(--list-items-width) var(--list-spacing) [content] 1fr;
height: var(--page-height);
    "#
    );
    let list_items_style = use_style!(
        r#"
grid-column: items-list;
display: flex;
flex-flow: column;
padding-right: var(--list-items-padding-right);
border-right: var(--list-items-border-width) solid var(--control-border-color);
box-sizing: border-box;
height: var(--page-height);
overflow-y: auto;

.cosmo-button {
	margin-top: auto;
}
"#
    );
    let list_content_style = use_style!(
        r#"
grid-column: content;
height: 100%;
overflow-y: auto;
    "#
    );

    let item_style = use_style!(
        r#"
--list-item-color: var(--black);

color: var(--list-item-color);
padding: var(--list-item-padding-top) var(--list-item-padding-right)
    var(--list-item-padding-bottom) var(--list-item-padding-left);
overflow-x: hidden;
white-space: nowrap;
text-overflow: ellipsis;
width: 100%;
box-sizing: border-box;
cursor: pointer;
min-height: var(--list-item-height);
border-radius: var(--border-radius);
background: var(--list-item-background);
display: flex;
align-items: center;

&:hover {
	--list-item-background: var(--control-border-color);
}
    "#
    );
    let item_active_style = use_style!(
        r#"
--list-item-background: var(--primary-color);
--list-item-color: var(--white);

font-weight: var(--font-weight-bold);

@media screen and (prefers-color-scheme: dark) {
    color: var(--black);
}

&:hover {
	--list-item-background: var(--primary-color);
	--list-item-color: var(--white);

	font-weight: var(--font-weight-bold);

    @media screen and (prefers-color-scheme: dark) {
		--list-item-color: var(--black);
    }
}

&::selection,
&:hover::selection {
	--list-item-background: var(--white);
	--list-item-color: var(--primary-color);
}
    "#
    );

    let selected_item_state = use_state_eq(|| {
        if !props.children.is_empty() {
            Some(0)
        } else {
            None
        }
    });
    let selected_idx = if let Some(selected_idx) = props.selected_index {
        selected_idx
    } else {
        (*selected_item_state).unwrap_or(0)
    };

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

                        if let Some(on_select) = props.on_select_item.clone() {
                            Callback::from(move|_| on_select.emit(idx))
                        } else {
                            Callback::from(move |_| {
                                selected_item_state.set(Some(idx));
                            })
                        }
                    };
                    let classes = if selected_idx == idx {
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
                if let Some((_, item)) = props.children.iter().enumerate().nth(selected_idx).clone() {
                    {item}
                }
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
        VChild::new(
            CosmoSideListItemProps {
                label,
                children: ChildrenRenderer::new(vec![children]),
            },
            None,
        )
    }
}
