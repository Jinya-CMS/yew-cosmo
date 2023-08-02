use stylist::yew::{styled_component, use_style};
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::VChild;

#[derive(Clone, derive_more::From, PartialEq)]
pub enum CosmoTabControlChildren {
    CosmoTabItem(VChild<CosmoTabItem>)
}

impl Into<Html> for CosmoTabControlChildren {
    fn into(self) -> Html {
        match self {
            CosmoTabControlChildren::CosmoTabItem(child) => child.into()
        }
    }
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTabControlProps {
    #[prop_or_default]
    pub children: ChildrenRenderer<CosmoTabControlChildren>,
}

#[styled_component(CosmoTabControl)]
pub fn tab_control(props: &CosmoTabControlProps) -> Html {
    let tab_style = use_style!(r#"
display: grid;
grid-template-rows: [tablist] 20px [tabcontent] 1fr;
max-height: 100%;
gap: 10px;
    "#);
    let tabs_style = use_style!(r#"
grid-row: tablist;
display: flex;
width: 100%;
justify-content: flex-end;
gap: 20px;
"#);
    let tab_content_style = use_style!(r#"
max-height: 100%;
overflow-y: auto;
grid-row: tabcontent;
    "#);
    let item_style = use_style!(r#"
text-transform: uppercase;
color: var(--menu-text-color);
font-size: 20px;
font-weight: var(--font-weight-bold);
height: 20px;
cursor: pointer;
    "#);
    let item_active_style = use_style!(r#"
color: var(--black);
    "#);

    let selected_item_state = use_state_eq(|| if props.children.len() > 0 { Some(0) } else { None });

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
            </nav>
            <div class={tab_content_style}>
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