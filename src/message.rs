use stylist::yew::{styled_component, use_style};
use yew::prelude::*;
use yew::virtual_dom::VNode;

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

    pub fn get_message_accent_color(&self) -> String {
        match self {
            CosmoMessageType::Information => "var(--information-color)",
            CosmoMessageType::Warning => "var(--warning-color)",
            CosmoMessageType::Positive => "var(--positive-color)",
            CosmoMessageType::Negative => "var(--negative-color)",
        }
        .to_string()
    }

    pub fn get_message_accent_color_light(&self) -> String {
        match self {
            CosmoMessageType::Information => "var(--information-color-light)",
            CosmoMessageType::Warning => "var(--warning-color-light)",
            CosmoMessageType::Positive => "var(--positive-color-light)",
            CosmoMessageType::Negative => "var(--negative-color-light)",
        }
        .to_string()
    }

    pub fn get_message_accent_color_lighter(&self) -> String {
        match self {
            CosmoMessageType::Information => "var(--information-color-lighter)",
            CosmoMessageType::Warning => "var(--warning-color-lighter)",
            CosmoMessageType::Positive => "var(--positive-color-lighter)",
            CosmoMessageType::Negative => "var(--negative-color-lighter)",
        }
        .to_string()
    }
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoMessageProps {
    #[prop_or_default]
    pub message_type: CosmoMessageType,
    #[prop_or(None)]
    pub header: Option<AttrValue>,
    pub message: AttrValue,
    #[prop_or(None)]
    pub actions: Option<VNode>,
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
    let accent_style = use_style!(
        r#"
--modal-accent-color: ${modal_accent_color};
--modal-accent-color-light: ${modal_accent_color_light};
--modal-accent-color-lighter: ${modal_accent_color_lighter};
    "#,
        modal_accent_color = props.message_type.get_message_accent_color(),
        modal_accent_color_light = props.message_type.get_message_accent_color_light(),
        modal_accent_color_lighter = props.message_type.get_message_accent_color_lighter(),
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
    let message_button_bar_style = use_style!(
        r#"
display: flex;
justify-content: flex-end;
width: 100%;
margin-top: var(--modal-button-bar-margin-top);
gap: var(--button-container-gap);

> .cosmo-button {
    border-left-width: var(--button-border-width);
}

.cosmo-button:last-of-type {
    --button-color: var(--white);
    --button-background: var(--modal-accent-color);
    --button-border-color: var(--modal-accent-color);
}

.cosmo-button:last-of-type:not(:disabled):hover,
.cosmo-button:last-of-type:not(:disabled):focus {
    --button-border-color: var(--modal-accent-color-light);
    --button-background: var(--modal-accent-color-light);
}

.cosmo-button:last-of-type:not(:disabled):active {
    --button-border-color: var(--modal-accent-color-lighter);
    --button-background: var(--modal-accent-color-lighter);
}

@media screen and (prefers-color-scheme: dark) {
    .cosmo-button:last-of-type {
        --button-color: var(--black);
    }

    .cosmo-button:last-of-type:not(:disabled):hover,
    .cosmo-button:last-of-type:not(:disabled):focus {
        --button-border-color: var(--primary-color-dark);
        --button-background: var(--primary-color-dark);
    }

    .cosmo-button:last-of-type:not(:disabled):active {
        --button-border-color: var(--primary-color-darker);
        --button-background: var(--primary-color-darker);
    }
}
    "#
    );

    html!(
        <div class={classes!(container_style, accent_style)}>
            if let Some(header) = props.header.clone() {
                <span class={header_style}>{header}</span>
            }
            <p class={message_style}>{props.message.clone()}</p>
            if let Some(actions) = props.actions.clone() {
                <div class={message_button_bar_style}>
                    {actions}
                </div>
            }
        </div>
    )
}
