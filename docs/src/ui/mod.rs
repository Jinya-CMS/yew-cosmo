use stylist::yew::{styled_component, use_style};
use yew::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoDemoProps {
    pub children: Children,
    #[prop_or(false)]
    pub flex: bool,
}

#[styled_component(CosmoDemo)]
pub fn cosmo_demo(props: &CosmoDemoProps) -> Html {
    let style = use_style!(
        r#"
border: 0.0625rem solid var(--primary-color);
padding: 2rem 1rem 1rem;
position: relative;
margin: 1rem 0;
border-radius: var(--border-radius);

::before {
	content: 'Demo';
	background: var(--primary-color);
	color: #ffffff;
	position: absolute;
	left: 0;
	top: 0;
	padding: 0.25rem 0.5rem;
	border-bottom-right-radius: var(--border-radius);
}
    "#
    );

    let mut flex_style = Some(use_style!(r#"
display: flex;
gap: 0.5rem;
"#));
    if !props.flex {
        flex_style = None;
    }

    html!(
        <div class={classes!(flex_style, style)}>{for props.children.iter()}</div>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoDocsPreProps {
    pub children: Children,
}

#[styled_component(CosmoDocsCodeSample)]
pub fn cosmo_docs_code_sample(props: &CosmoDocsPreProps) -> Html {
    let style = use_style!(
        r#"
border: 1px solid var(--primary-color);
padding: 2rem 1rem 1rem;
position: relative;
margin: 1rem 0;
border-radius: var(--border-radius);
margin: unset;
overflow: auto;
font-family: var(--font-family-code);

::before {
	content: 'Code';
	background: var(--primary-color);
	color: #ffffff;
	position: absolute;
	left: 0;
	top: 0;
	padding: 0.25rem 0.5rem;
	border-bottom-right-radius: var(--border-radius);
}
    "#
    );

    let details_style = use_style!(r#"
&[open] summary::before {
	transform: rotate(90deg);
}
"#);
    let summary_style = use_style!(r#"
font-size: 125%;
cursor: pointer;
list-style-type: none;
display: flex;
align-items: center;

&::before {
	content: '';
	background-image: url("data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyNCAyNCI+PHBhdGggc3Ryb2tlPSJjdXJyZW50Q29sb3IiIGQ9Ik04LjU5LDE2LjU4TDEzLjE3LDEyTDguNTksNy40MUwxMCw2TDE2LDEyTDEwLDE4TDguNTksMTYuNThaIiAvPjwvc3ZnPg==");
	display: inline-block;
	width: 1em;
	height: 1em;
	transition: transform var(--transition-duration);
}
	"#);

    html!(
        <details class={details_style}>
            <summary class={summary_style}>{"Show code sample"}</summary>
            <pre class={classes!(style, "is--dark")}>{for props.children.iter()}</pre>
        </details>
    )
}
