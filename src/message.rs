use stylist::yew::{styled_component, use_style};
use yew::prelude::*;

#[derive(PartialEq, Clone, Default)]
pub enum CosmoMessageType {
    #[default]
    Information,
    Warning,
    Positive,
    Negative,
}

impl CosmoMessageType {
    pub(crate) fn get_background(&self) -> AttrValue {
        AttrValue::from(match self {
            CosmoMessageType::Information => "var(--information-color-alpha-25)",
            CosmoMessageType::Warning => "var(--warning-color-alpha-25)",
            CosmoMessageType::Positive => "var(--positive-color-alpha-25)",
            CosmoMessageType::Negative => "var(--negative-color-alpha-25)",
        })
    }

    pub(crate) fn get_border(&self) -> AttrValue {
        AttrValue::from(match self {
            CosmoMessageType::Information => "var(--information-color)",
            CosmoMessageType::Warning => "var(--warning-color)",
            CosmoMessageType::Positive => "var(--positive-color)",
            CosmoMessageType::Negative => "var(--negative-color)",
        })
    }
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoMessageProps {
    #[prop_or_default]
    pub message_type: CosmoMessageType,
    #[prop_or(None)]
    pub header: Option<AttrValue>,
    pub message: AttrValue,
}

#[styled_component(CosmoMessage)]
pub fn message(props: &CosmoMessageProps) -> Html {
    let container_style = use_style!(
        r#"
--message-background: ${background};
--message-border: ${border_color};

width: 100%;
background: var(--message-background);
color: var(--black);
border-top: var(--message-border-top-width) solid var(--message-border);
padding: var(--message-padding-top) var(--message-padding-right) var(--message-padding-bottom)
    var(--message-padding-left);
margin-bottom: var(--message-margin-bottom);
box-sizing: border-box;
border-radius: var(--border-radius);
backdrop-filter: var(--message-backdrop-filter);

&:selection {
	background: var(--message-border);
	color: var(--white);
}
    "#,
        background = props.message_type.get_background(),
        border_color = props.message_type.get_border()
    );
    let header_style = use_style!(
        r#"
margin: 0;
color: var(--black);
font-weight: var(--font-weight-light);
font-family: var(--font-family-heading);
font-size: var(--message-header-font-size);
display: block;

&::selection {
	background: var(--message-border);
	color: var(--white);
}
    "#
    );
    let message_style = use_style!(
        r#"
margin: 0;

&::selection {
	background: var(--message-border);
	color: var(--white);
}
    "#
    );

    html!(
        <div class={container_style}>
            if let Some(header) = props.header.clone() {
                <span class={header_style}>{header}</span>
            }
            <p class={message_style}>{props.message.clone()}</p>
        </div>
    )
}
