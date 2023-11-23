use stylist::yew::{styled_component, use_style};
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_hooks::use_unmount;

use crate::button::CosmoButton;
use crate::prelude::CosmoTheme;

#[derive(PartialEq, Clone, Default)]
pub enum CosmoModalType {
    #[default]
    Primary,
    Information,
    Warning,
    Positive,
    Negative,
}

pub type CosmoAlertType = CosmoModalType;

impl CosmoModalType {
    pub fn get_modal_accent_color(&self) -> String {
        match self {
            CosmoModalType::Primary => "var(--primary-color)",
            CosmoModalType::Information => "var(--information-color)",
            CosmoModalType::Warning => "var(--warning-color)",
            CosmoModalType::Positive => "var(--positive-color)",
            CosmoModalType::Negative => "var(--negative-color)",
        }
            .to_string()
    }

    pub fn get_modal_accent_color_light(&self) -> String {
        match self {
            CosmoModalType::Primary => "var(--primary-color-light)",
            CosmoModalType::Information => "var(--information-color-light)",
            CosmoModalType::Warning => "var(--warning-color-light)",
            CosmoModalType::Positive => "var(--positive-color-light)",
            CosmoModalType::Negative => "var(--negative-color-light)",
        }
            .to_string()
    }

    pub fn get_modal_accent_color_lighter(&self) -> String {
        match self {
            CosmoModalType::Primary => "var(--primary-color-lighter)",
            CosmoModalType::Information => "var(--information-color-lighter)",
            CosmoModalType::Warning => "var(--warning-color-lighter)",
            CosmoModalType::Positive => "var(--positive-color-lighter)",
            CosmoModalType::Negative => "var(--negative-color-lighter)",
        }
            .to_string()
    }
}

impl ToString for CosmoModalType {
    fn to_string(&self) -> String {
        match self {
            CosmoModalType::Primary => "primary",
            CosmoModalType::Information => "information",
            CosmoModalType::Warning => "warning",
            CosmoModalType::Positive => "positive",
            CosmoModalType::Negative => "negative",
        }
            .into()
    }
}

impl From<CosmoModalType> for AttrValue {
    fn from(value: CosmoModalType) -> Self {
        value.to_string().into()
    }
}

impl From<String> for CosmoModalType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "information" => Self::Information,
            "warning" => Self::Warning,
            "positive" => Self::Positive,
            "negative" => Self::Negative,
            _ => Self::Primary,
        }
    }
}

impl From<AttrValue> for CosmoModalType {
    fn from(value: AttrValue) -> Self {
        Self::from(value.to_string())
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct CosmoModalProps {
    #[prop_or_default]
    pub children: Children,
    pub buttons: VNode,
    pub title: AttrValue,
    #[prop_or(false)]
    pub is_form: bool,
    #[prop_or_default]
    pub on_form_submit: Option<Callback<()>>,
    #[prop_or_default]
    pub theme: CosmoTheme,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub modal_type: CosmoModalType
}

#[styled_component(CosmoModal)]
pub fn modal(props: &CosmoModalProps) -> Html {
    let modal_id = use_state_eq(|| uuid::Uuid::new_v4().to_string());

    let modal_container_style = use_style!(
        r#"
position: fixed;
top: 0;
left: 0;
right: 0;
bottom: 0;
background: var(--modal-backdrop);
height: 100vh;
width: 100vw;
backdrop-filter: var(--modal-container-backdrop-filter);
display: flex;
justify-content: center;
align-items: center;
z-index: 9999;
border: none;
color: var(--black);
    "#
    );
    let modal_style = use_style!(
        r#"
border: var(--modal-border-width) solid var(--modal-accent-color);
padding: var(--modal-padding-top) var(--modal-padding-right) var(--modal-padding-bottom) var(--modal-padding-left);
min-width: var(--modal-min-width);
box-sizing: border-box;
border-radius: var(--border-radius);
backdrop-filter: var(--modal-backdrop-filter);
background: var(--modal-background);

&::before {
    content: '';
    position: absolute;
    width: var(--modal-bar-width);
    height: var(--modal-bar-height);
    background: var(--modal-accent-color);
    border-radius: var(--border-radius);
}
    "#
    );
    let accent_style = use_style!(
        r#"
--modal-accent-color: ${modal_accent_color};
--modal-accent-color-light: ${modal_accent_color_light};
--modal-accent-color-lighter: ${modal_accent_color_lighter};
    "#,
        modal_accent_color = props.modal_type.get_modal_accent_color(),
        modal_accent_color_light = props.modal_type.get_modal_accent_color_light(),
        modal_accent_color_lighter = props.modal_type.get_modal_accent_color_lighter(),
    );
    let modal_title_style = use_style!(
        r#"
padding: 0;
margin: var(--modal-title-margin-top) var(--modal-title-margin-right) var(--modal-title-margin-bottom) var(--modal-title-margin-left);
text-transform: uppercase;
font-size: var(--modal-title-font-size);
line-height: var(--modal-title-font-size);
height: var(--modal-title-font-size);
vertical-align: text-top;
font-weight: var(--font-weight-normal);
font-family: var(--font-family-modal-title);
    "#
    );
    let modal_content_style = use_style!(
        r#"
font-weight: var(--font-weight-normal);
font-family: var(--font-family);
padding: 0;
margin: 0;
    "#
    );
    let modal_button_bar_style = use_style!(
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

    let on_submit = props.on_form_submit.clone().map(move |on_submit| {
        Callback::from(move |evt: SubmitEvent| {
            evt.prevent_default();
            on_submit.emit(());
        })
    });
    let tag = if props.is_form { "form" } else { "div" };

    let modal_host = if let Some(modal_host) =
        gloo::utils::document().get_element_by_id((*modal_id).clone().as_str())
    {
        modal_host
    } else {
        let modal_host = gloo::utils::document()
            .create_element("div")
            .expect("Failed to create div");
        modal_host.set_id((*modal_id).clone().as_str());
        gloo::utils::body()
            .append_child(&modal_host)
            .expect("Failed to append child");
        modal_host
    };
    {
        let modal_host = modal_host.clone();
        use_unmount(move || modal_host.remove());
    }

    create_portal(
        html!(
            <dialog class={classes!(modal_container_style, accent_style, props.theme.clone(), props.classes.clone())} open={true}>
                <@{tag} class={modal_style} onsubmit={on_submit}>
                    <h1 class={modal_title_style}>{props.title.clone()}</h1>
                    <div class={modal_content_style}>
                        {for props.children.iter()}
                    </div>
                    <div class={modal_button_bar_style}>
                        {props.buttons.clone()}
                    </div>
                </@>
            </dialog>
        ),
        modal_host,
    )
}

#[derive(Properties, PartialEq, Clone)]
pub struct CosmoAlertProps {
    pub title: AttrValue,
    pub message: AttrValue,
    pub close_label: AttrValue,
    pub on_close: Callback<()>,
    #[prop_or_default]
    pub theme: CosmoTheme,
    #[prop_or_default]
    pub alert_type: CosmoModalType,
}

#[styled_component(CosmoAlert)]
pub fn alert(props: &CosmoAlertProps) -> Html {
    let on_close = use_callback(props.on_close.clone(), |_, on_close| on_close.emit(()));

    let message_style = use_style!(
        r#"
white-space: pre-wrap;
    "#
    );

    html!(
        <CosmoModal modal_type={props.alert_type.clone()} theme={props.theme.clone()} title={props.title.clone()} buttons={html!(<CosmoButton on_click={on_close} label={props.close_label.clone()} />)}>
            <div class={message_style}>{props.message.clone()}</div>
        </CosmoModal>
    )
}

#[derive(Properties, PartialEq, Clone)]
pub struct CosmoConfirmProps {
    pub title: AttrValue,
    pub message: AttrValue,
    pub confirm_label: AttrValue,
    pub decline_label: AttrValue,
    pub on_confirm: Callback<()>,
    pub on_decline: Callback<()>,
    #[prop_or_default]
    pub theme: CosmoTheme,
    #[prop_or_default]
    pub confirm_type: CosmoModalType,
}

#[styled_component(CosmoConfirm)]
pub fn confirm(props: &CosmoConfirmProps) -> Html {
    let on_confirm = use_callback(props.on_confirm.clone(), |_, callback| callback.emit(()));
    let on_decline = use_callback(props.on_decline.clone(), |_, callback| callback.emit(()));

    let message_style = use_style!(
        r#"
white-space: pre-wrap;
    "#
    );

    html!(
        <CosmoModal modal_type={props.confirm_type.clone()} theme={props.theme.clone()} title={props.title.clone()} buttons={html!(
            <>
                <CosmoButton on_click={on_decline} label={props.decline_label.clone()} />
                <CosmoButton on_click={on_confirm} label={props.confirm_label.clone()} />
            </>
        )}>
            <div class={message_style}>{props.message.clone()}</div>
        </CosmoModal>
    )
}
