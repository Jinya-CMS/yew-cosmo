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
            CosmoMessageType::Information => "var(--information-light-color)",
            CosmoMessageType::Warning => "var(--warning-light-color)",
            CosmoMessageType::Positive => "var(--positive-light-color)",
            CosmoMessageType::Negative => "var(--negative-light-color)",
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
    let container_style = use_style!(r#"
width: 100%;
background: ${background};
color: var(--black);
border-top: 4px solid ${border_color};
padding: 8px 16px;
margin-bottom: 16px;
box-sizing: border-box;

&::selection {
    background: ${border_color};
    color: var(--white);
}
    "#,
    background = props.message_type.get_background(),
    border_color = props.message_type.get_border());
    let header_style = use_style!(r#"
margin: 0;
color: var(--black);
font-weight: var(--font-weight-light);
font-size: 24px;
display: block;

&::selection {
    background: ${border_color};
    color: var(--white);
}
    "#,
    border_color = props.message_type.get_border());
    let message_style = use_style!(r#"
margin: 0;

&::selection {
    background: ${border_color};
    color: var(--white);
}
    "#,
    border_color = props.message_type.get_border());

    html!(
        <div class={container_style}>
            if let Some(header) = props.header.clone() {
                <span class={header_style}>{header}</span>
            }
            <p class={message_style}>{props.message.clone()}</p>
        </div>
    )
}