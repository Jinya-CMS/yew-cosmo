use yew::prelude::*;

use yew_cosmo::prelude::*;

use crate::ui::{CosmoDemo, CosmoDocsCodeSample};

#[function_component(Toolbar)]
pub fn toolbar() -> Html {
    html!(
        <>
            <CosmoTitle title="Toolbar" />
            <CosmoParagraph>
                {"The toolbar control can be used to group buttons into logic segments and allow the user to easier grasp what belongs to each other."}
            </CosmoParagraph>
            <CosmoDemo>
                <CosmoToolbar>
                    <CosmoToolbarGroup>
                        <CosmoButton label="Toolbar button 1" />
                        <CosmoButton label="Toolbar button 2" />
                    </CosmoToolbarGroup>
                    <CosmoToolbarGroup>
                        <CosmoButton label="Toolbar button 3" />
                        <CosmoButton label="Toolbar button 4" />
                    </CosmoToolbarGroup>
                </CosmoToolbar>
            </CosmoDemo>
            <CosmoDocsCodeSample>{r#"<CosmoToolbar>
    <CosmoToolbarGroup>
        <CosmoButton label="Toolbar button 1" />
        <CosmoButton label="Toolbar button 2" />
    </CosmoToolbarGroup>
    <CosmoToolbarGroup>
        <CosmoButton label="Toolbar button 3" />
        <CosmoButton label="Toolbar button 4" />
    </CosmoToolbarGroup>
</CosmoToolbar>"#}</CosmoDocsCodeSample>
        </>
    )
}
