use stylist::yew::{styled_component, use_style};
use yew::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoToolbarProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<CosmoToolbarGroup>,
}

#[styled_component(CosmoToolbar)]
pub fn toolbar(props: &CosmoToolbarProps) -> Html {
    let toolbar_style = use_style!(
        r#"
display: flex;
gap: 16px;
    "#
    );

    html!(
        <div class={toolbar_style}>
            {for props.children.iter()}
        </div>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoToolbarGroupProps {
    #[prop_or_default]
    pub children: Children,
}

#[styled_component(CosmoToolbarGroup)]
pub fn toolbar(props: &CosmoToolbarGroupProps) -> Html {
    let toolbar_group_style = use_style!(
        r#"
display: flex;
align-items: center;

.cosmo-button:not(:first-child) {
    border-left-width: 0px;
}
    "#
    );

    html!(
        <div class={toolbar_group_style}>
            {for props.children.iter()}
        </div>
    )
}
