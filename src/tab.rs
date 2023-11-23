use stylist::yew::{styled_component, use_style};
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::VChild;

#[derive(Clone, derive_more::From, PartialEq)]
pub enum CosmoTabControlChildren {
    CosmoTabItem(VChild<CosmoTabItem>),
}

#[allow(clippy::from_over_into)]
impl Into<Html> for CosmoTabControlChildren {
    fn into(self) -> Html {
        match self {
            CosmoTabControlChildren::CosmoTabItem(child) => child.into(),
        }
    }
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTabControlProps {
    #[prop_or_default]
    pub children: ChildrenRenderer<CosmoTabControlChildren>,
    #[prop_or_default]
    pub selected_index: Option<usize>,
    #[prop_or_default]
    pub on_select_item: Option<Callback<usize>>,
}

#[styled_component(CosmoTabControl)]
pub fn tab_control(props: &CosmoTabControlProps) -> Html {
    let tab_style = use_style!(
        r#"
display: grid;
grid-template-rows: [tablist] var(--tab-links-height) [tabcontent] 1fr;
max-height: 100%;
gap: var(--tab-gap);
    "#
    );
    let tabs_style = use_style!(
        r#"
grid-row: tablist;
display: flex;
width: 100%;
justify-content: flex-end;
gap: var(--tab-links-gap);
"#
    );
    let tab_content_style = use_style!(
        r#"
max-height: 100%;
overflow-y: auto;
grid-row: tabcontent;
    "#
    );
    let item_style = use_style!(
        r#"
text-transform: uppercase;
color: var(--menu-text-color);
font-size: var(--tab-link-font-size);
font-weight: var(--font-weight-bold);
height: var(--tab-link-font-size);
cursor: pointer;
    "#
    );
    let item_active_style = use_style!(
        r#"
color: var(--black);
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
        <div class={tab_style}>
            <nav class={tabs_style}>
                {for props.children.iter().enumerate().map(|(idx, child)| {
                    let CosmoTabControlChildren::CosmoTabItem(child) = child;
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
                    let selected_idx = if let Some(selected_idx) = props.selected_index {
                        selected_idx
                    } else {
                        (*selected_item_state).unwrap_or(0)
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
            </nav>
            <div class={tab_content_style}>
                if let Some((_, item)) = props.children.iter().enumerate().nth(selected_idx).clone() {
                    {item}
                }
            </div>
        </div>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTabItemProps {
    pub label: AttrValue,
    #[prop_or_default]
    pub children: Children,
}

#[styled_component(CosmoTabItem)]
pub fn tab_item(props: &CosmoTabItemProps) -> Html {
    html!(
        {for props.children.iter()}
    )
}

impl CosmoTabItem {
    pub fn new(props: CosmoTabItemProps) -> VChild<Self> {
        VChild::new(props, None)
    }

    pub fn from_label_and_children(label: AttrValue, children: Html) -> VChild<Self> {
        VChild::new(
            CosmoTabItemProps {
                label,
                children: ChildrenRenderer::new(vec![children]),
            },
            None,
        )
    }
}
