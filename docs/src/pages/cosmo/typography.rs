use yew::prelude::*;

use yew_cosmo::prelude::*;

use crate::ui::CosmoDemo;
use crate::ui::CosmoDocsPre;

#[function_component(Typography)]
pub fn typography() -> Html {
    html!(
        <>
            <CosmoTitle title="Typography" />
            <CosmoParagraph>
                {"Due to Cosmo CSS being targeted for web applications, it only features the basic
                typographic needs."}
            </CosmoParagraph>
            <CosmoHeader level={CosmoHeaderLevel::H2} header="Supported tags" />
            <CosmoDemo>
                <CosmoHeader level={CosmoHeaderLevel::H1} header="Heading level 1" />
                <CosmoHeader level={CosmoHeaderLevel::H2} header="Heading level 2" />
                <CosmoHeader level={CosmoHeaderLevel::H3} header="Heading level 3" />
                <CosmoHeader level={CosmoHeaderLevel::H4} header="Heading level 4" />
                <CosmoHeader level={CosmoHeaderLevel::H5} header="Heading level 5" />
                <CosmoHeader level={CosmoHeaderLevel::H6} header="Heading level 6" />
            </CosmoDemo>
            <CosmoDocsPre>{r#"<CosmoHeader level={CosmoHeaderLevel::H1} header="Heading level 1" />
<CosmoHeader level={CosmoHeaderLevel::H2} header="Heading level 2" />
<CosmoHeader level={CosmoHeaderLevel::H3} header="Heading level 3" />
<CosmoHeader level={CosmoHeaderLevel::H4} header="Heading level 4" />
<CosmoHeader level={CosmoHeaderLevel::H5} header="Heading level 5" />
<CosmoHeader level={CosmoHeaderLevel::H6} header="Heading level 6" />"#}</CosmoDocsPre>
            <CosmoHr />
            <CosmoDemo>
                <CosmoParagraph>{"Paragraph"}</CosmoParagraph>
            </CosmoDemo>
            <CosmoDocsPre>{r#"<CosmoParagraph>{"Paragraph"}</CosmoParagraph>"#}</CosmoDocsPre>
            <CosmoHr />
            <CosmoDemo>
                <CosmoAnchor href="#">{"Anchor tag"}</CosmoAnchor><CosmoBr />
                <CosmoAnchor href="#">{"Anchor tag with Route"}</CosmoAnchor><CosmoBr />
                <CosmoStrong>{"Strong tag"}</CosmoStrong><CosmoBr />
                <CosmoEm>{"Emphasis tag"}</CosmoEm><CosmoBr />
                <CosmoCode>{"Code tag"}</CosmoCode>
            </CosmoDemo>
            <CosmoDocsPre>{r#"<CosmoAnchor href="">{"Anchor tag"}</CosmoAnchor>
<CosmoAnchorLink<Route> to={Route::Home}>{"Anchor tag with Route"}</CosmoAnchorLink<Route>>
<CosmoStrong>{"Strong tag"}</CosmoStrong>
<CosmoEm>{"Emphasis tag"}</CosmoEm>
<CosmoCode>{"Code tag"}</CosmoCode>"#}</CosmoDocsPre>
            <CosmoHeader level={CosmoHeaderLevel::H2} header="Tables" />
            <CosmoParagraph>{"Tables are used to show data that you can display any amount of data."}</CosmoParagraph>
            <CosmoDemo>
                <CosmoTable headers={vec![AttrValue::from("Column name"), AttrValue::from("Type"), AttrValue::from("Allow null"), AttrValue::from("Keys"), AttrValue::from("Default value"), AttrValue::from("Extra data")]}>
                    <CosmoTableRow>
                        <CosmoTableCell>{"api_key"}</CosmoTableCell>
                        <CosmoTableCell>{"varchar(255)"}</CosmoTableCell>
                        <CosmoTableCell>{"NO"}</CosmoTableCell>
                        <CosmoTableCell>{"PRI"}</CosmoTableCell>
                        <CosmoTableCell>{"null"}</CosmoTableCell>
                        <CosmoTableCell></CosmoTableCell>
                    </CosmoTableRow>
                    <CosmoTableRow>
                        <CosmoTableCell>{"user_id"}</CosmoTableCell>
                        <CosmoTableCell>{"int"}</CosmoTableCell>
                        <CosmoTableCell>{"YES"}</CosmoTableCell>
                        <CosmoTableCell>{"MUL"}</CosmoTableCell>
                        <CosmoTableCell>{"null"}</CosmoTableCell>
                        <CosmoTableCell></CosmoTableCell>
                    </CosmoTableRow>
                    <CosmoTableRow>
                        <CosmoTableCell>{"valid_since"}</CosmoTableCell>
                        <CosmoTableCell>{"datetime"}</CosmoTableCell>
                        <CosmoTableCell>{"NO"}</CosmoTableCell>
                        <CosmoTableCell></CosmoTableCell>
                        <CosmoTableCell>{"null"}</CosmoTableCell>
                        <CosmoTableCell></CosmoTableCell>
                    </CosmoTableRow>
                    <CosmoTableRow>
                        <CosmoTableCell>{"user_agent"}</CosmoTableCell>
                        <CosmoTableCell>{"varchar(255)"}</CosmoTableCell>
                        <CosmoTableCell>{"YES"}</CosmoTableCell>
                        <CosmoTableCell></CosmoTableCell>
                        <CosmoTableCell>{"null"}</CosmoTableCell>
                        <CosmoTableCell></CosmoTableCell>
                    </CosmoTableRow>
                    <CosmoTableRow>
                        <CosmoTableCell>{"remote_address"}</CosmoTableCell>
                        <CosmoTableCell>{"varchar(255)"}</CosmoTableCell>
                        <CosmoTableCell>{"YES"}</CosmoTableCell>
                        <CosmoTableCell></CosmoTableCell>
                        <CosmoTableCell>{"null"}</CosmoTableCell>
                        <CosmoTableCell></CosmoTableCell>
                    </CosmoTableRow>
                </CosmoTable>
            </CosmoDemo>
            <CosmoDocsPre>{r#"<CosmoTable headers={vec![AttrValue::from("Column name"), AttrValue::from("Type"), AttrValue::from("Allow null"), AttrValue::from("Keys"), AttrValue::from("Default value"), AttrValue::from("Extra data")]}>
    <CosmoTableRow>
        <CosmoTableCell>{"api_key"}</CosmoTableCell>
        <CosmoTableCell>{"varchar(255)"}</CosmoTableCell>
        <CosmoTableCell>{"NO"}</CosmoTableCell>
        <CosmoTableCell>{"PRI"}</CosmoTableCell>
        <CosmoTableCell>{"null"}</CosmoTableCell>
        <CosmoTableCell></CosmoTableCell>
    </CosmoTableRow>
    <CosmoTableRow>
        <CosmoTableCell>{"user_id"}</CosmoTableCell>
        <CosmoTableCell>{"int"}</CosmoTableCell>
        <CosmoTableCell>{"YES"}</CosmoTableCell>
        <CosmoTableCell>{"MUL"}</CosmoTableCell>
        <CosmoTableCell>{"null"}</CosmoTableCell>
        <CosmoTableCell></CosmoTableCell>
    </CosmoTableRow>
    <CosmoTableRow>
        <CosmoTableCell>{"valid_since"}</CosmoTableCell>
        <CosmoTableCell>{"datetime"}</CosmoTableCell>
        <CosmoTableCell>{"NO"}</CosmoTableCell>
        <CosmoTableCell></CosmoTableCell>
        <CosmoTableCell>{"null"}</CosmoTableCell>
        <CosmoTableCell></CosmoTableCell>
    </CosmoTableRow>
    <CosmoTableRow>
        <CosmoTableCell>{"user_agent"}</CosmoTableCell>
        <CosmoTableCell>{"varchar(255)"}</CosmoTableCell>
        <CosmoTableCell>{"YES"}</CosmoTableCell>
        <CosmoTableCell></CosmoTableCell>
        <CosmoTableCell>{"null"}</CosmoTableCell>
        <CosmoTableCell></CosmoTableCell>
    </CosmoTableRow>
    <CosmoTableRow>
        <CosmoTableCell>{"remote_address"}</CosmoTableCell>
        <CosmoTableCell>{"varchar(255)"}</CosmoTableCell>
        <CosmoTableCell>{"YES"}</CosmoTableCell>
        <CosmoTableCell></CosmoTableCell>
        <CosmoTableCell>{"null"}</CosmoTableCell>
        <CosmoTableCell></CosmoTableCell>
    </CosmoTableRow>
</CosmoTable>"#}</CosmoDocsPre>
            <CosmoHeader level={CosmoHeaderLevel::H2} header="Key value list" />
            <CosmoParagraph>{"A key value list displays information based on a key-value pattern."}</CosmoParagraph>
            <CosmoDemo>
                <CosmoKeyValueList>
                    <CosmoKeyValueListItem title="Server type">{"MySQL Community Server - GPL"}</CosmoKeyValueListItem>
                    <CosmoKeyValueListItem title="Server version">{"8.0.25"}</CosmoKeyValueListItem>
                    <CosmoKeyValueListItem title="Compile machine">{"x86_64"}</CosmoKeyValueListItem>
                    <CosmoKeyValueListItem title="Compile operating system">{"Linux"}</CosmoKeyValueListItem>
                </CosmoKeyValueList>
            </CosmoDemo>
            <CosmoDocsPre>{r#"<CosmoKeyValueList>
    <CosmoKeyValueListItem title="Server type">{"MySQL Community Server - GPL"}</CosmoKeyValueListItem>
    <CosmoKeyValueListItem title="Server version">{"8.0.25"}</CosmoKeyValueListItem>
    <CosmoKeyValueListItem title="Compile machine">{"x86_64"}</CosmoKeyValueListItem>
    <CosmoKeyValueListItem title="Compile operating system">{"Linux"}</CosmoKeyValueListItem>
</CosmoKeyValueList>"#}</CosmoDocsPre>
        </>
    )
}
