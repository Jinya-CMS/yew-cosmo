use bounce::helmet::Helmet;
use stylist::yew::use_style;
use yew::prelude::*;

use yew_cosmo::prelude::*;

#[function_component(Showcase)]
pub fn showcase() -> Html {
    let body_style = use_style!(r#"
* {
    outline: 1px dashed var(--black);
}"#);

    html!(
        <>
            <Helmet>
                <body class={body_style} />
            </Helmet>
            <CosmoTitle title="Layout Showcase" />
        </>
    )
}