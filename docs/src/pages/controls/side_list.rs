use yew::prelude::*;

use yew_cosmo::prelude::*;

use crate::ui::CosmoDocsPre;

fn with_size(size: usize) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::with_capacity(size);
    for i in 0..size {
        res.push((i + 1) as i32);
    }

    res
}

#[function_component(SideList)]
pub fn side_list() -> Html {
    let additional_items_state = use_state_eq(|| vec![1]);
    let on_click = use_callback(
        |_, state| state.set(with_size((*state).len() + 1)),
        additional_items_state.clone(),
    );

    html!(
        <CosmoSideList has_add_button={true} add_button_label="Add item" add_button_on_click={on_click}>
            <CosmoSideListItem label="About">
                <CosmoTitle title="About the side list" />
                <CosmoParagraph>
                    {"The Master-Detail list is a control to create a basic master detail view. At the bottom of the list you can add a button to give your users the option to create a new entry."}
                </CosmoParagraph>
                <CosmoParagraph>
                    {"The Master-Detail list is a top level component and can not be put inside a tab control. Due to the fact that Cosmo CSS is a pure CSS library you need to implement the master detail logic yourself."}
                </CosmoParagraph>
            </CosmoSideListItem>
            <CosmoSideListItem label="Code sample">
                <CosmoTitle title="Code sample" />
                <CosmoDocsPre>{r#"<CosmoSideList has_add_button={true} add_button_label="Add item" add_button_on_click={on_click}>
    <CosmoSideListItem label="About">
        <CosmoTitle title="About the side list" />
        <CosmoParagraph>
            {"The Master-Detail list is a control to create a basic master detail view. At the bottom of the list you can add a button to give your users the option to create a new entry."}
        </CosmoParagraph>
        <CosmoParagraph>
            {"The Master-Detail list is a top level component and can not be put inside a tab control. Due to the fact that Cosmo CSS is a pure CSS library you need to implement the master detail logic yourself."}
        </CosmoParagraph>
    </CosmoSideListItem>
    <CosmoSideListItem label="Code sample">
        <CosmoTitle title="Code sample" />
    </CosmoSideListItem>
    {for (*additional_items_state).iter().map(|idx|
        CosmoSideListItem::from_label_and_children(
            AttrValue::from(format!("Additional page {idx}")),
            html!(<CosmoTitle title={format!("Hello World!")} subtitle={format!("From the additional page {idx}")} />)
        )
    )}
</CosmoSideList>"#}</CosmoDocsPre>
            </CosmoSideListItem>
            {for (*additional_items_state).iter().map(|idx|
                CosmoSideListItem::from_label_and_children(
                    AttrValue::from(format!("Additional page {idx}")),
                    html!(<CosmoTitle title={format!("Hello World!")} subtitle={format!("From the additional page {idx}")} />)
                )
            )}
        </CosmoSideList>
    )
}
