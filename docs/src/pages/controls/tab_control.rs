use yew::prelude::*;

use yew_cosmo::prelude::*;

use crate::ui::{CosmoDemo, CosmoDocsCodeSample};

#[function_component(TabControl)]
pub fn tab_control() -> Html {
    html!(
        <>
            <CosmoTitle title="Tab control" />
            <CosmoParagraph>
                {"The tab control is a basic control to add additional navigation layers into your application. It can be nested inside a Master-Detail list."}
            </CosmoParagraph>
            <CosmoDemo>
                <CosmoTabControl>
                    <CosmoTabItem label="Page 1">
                        <CosmoParagraph>
                            {"This is the first page."}
                        </CosmoParagraph>
                    </CosmoTabItem>
                    <CosmoTabItem label="Page 2">
                        <CosmoParagraph>
                            {"This is the second page."}
                        </CosmoParagraph>
                    </CosmoTabItem>
                    <CosmoTabItem label="Page 3">
                        <CosmoParagraph>
                            {"This is the third page."}
                        </CosmoParagraph>
                    </CosmoTabItem>
                </CosmoTabControl>
            </CosmoDemo>
            <CosmoDocsCodeSample>{r#"<CosmoTabControl>
    <CosmoTabItem label="Page 1">
        <CosmoParagraph>
            {"This is the first page."}
        </CosmoParagraph>
    </CosmoTabItem>
    <CosmoTabItem label="Page 2">
        <CosmoParagraph>
            {"This is the second page."}
        </CosmoParagraph>
    </CosmoTabItem>
    <CosmoTabItem label="Page 3">
        <CosmoParagraph>
            {"This is the third page."}
        </CosmoParagraph>
    </CosmoTabItem>
</CosmoTabControl>"#}</CosmoDocsCodeSample>
        </>
    )
}
