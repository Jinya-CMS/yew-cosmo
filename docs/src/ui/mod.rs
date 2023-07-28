use stylist::yew::{styled_component, use_style};
use yew::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoDemoProps{
    pub children: Children
}

#[styled_component(CosmoDemo)]
pub fn cosmo_demo(props: &CosmoDemoProps)->Html {
    let style = use_style!(r#"
border: 1px solid var(--primary-color);
padding: 32px 16px 16px;
position: relative;
margin: 16px 0;

::before {
	content: 'Demo';
	background: var(--primary-color);
	color: var(--white);
	position: absolute;
	left: 0;
	top: 0;
	padding: 4px 8px;
}
    "#);

    html!(
        <div class={style}>{for props.children.iter()}</div>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoDocsPreProps{
    pub children: Children
}

#[styled_component(CosmoDocsPre)]
pub fn cosmo_docs_pre(props: &CosmoDocsPreProps)->Html {
    let style = use_style!(r#"
border: 1px solid var(--primary-color);
padding: 32px 16px 16px;
position: relative;
margin: 16px 0;
font-family: "Source Code Pro", monospace;

::before {
	content: 'Code';
	background: var(--primary-color);
	color: var(--black);
	position: absolute;
	left: 0;
	top: 0;
	padding: 4px 8px;
}
    "#);

    html!(
        <pre class={classes!(style, "cosmo--dark-theme")}>{for props.children.iter()}</pre>
    )
}