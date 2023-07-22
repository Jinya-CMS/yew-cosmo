use stylist::yew::{styled_component, use_style};
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_hooks::use_unmount;

use crate::button::CosmoButton;

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
}

#[styled_component(CosmoModal)]
pub fn modal(props: &CosmoModalProps) -> Html {
    let modal_id = use_state_eq(|| uuid::Uuid::new_v4().to_string());

    let modal_style = use_style!(r#"
border: 1px solid var(--primary-color);
background: linear-gradient(to top, var(--white) 0%, var(--white) 80%, var(--gradient-top-color) 100%);
padding: 24px 32px;
min-width: 274px;
box-sizing: border-box;

&::before {
    content: '';
    position: absolute;
    width: 208px;
    height: 8px;
    background: var(--primary-color);
}
    "#);
    let modal_title_style = use_style!(r#"
padding: 0;
margin: 18px 0 10px;
text-transform: uppercase;
font-size: 36px;
line-height: 36px;
height: 36px;
vertical-align: text-top;
font-weight: var(--font-weight-normal);
font-family: var(--font-family);
    "#);
    let modal_content_style = use_style!(r#"
font-weight: var(--font-weight-normal);
font-family: var(--font-family);
padding: 0;
margin: 0;
    "#);
    let modal_button_bar_style = use_style!(r#"
display: flex;
justify-content: flex-end;
width: 100%;
margin-top: 10px;
gap: 16px;

> .cosmo-button {
    border-left-width: 1px;
}
    "#);

    let modal_content = html!(
        <>
            <h1 class={modal_title_style}>{props.title.clone()}</h1>
            <div class={modal_content_style}>
                {for props.children.iter()}
            </div>
            <div class={modal_button_bar_style}>
                {props.buttons.clone()}
            </div>
        </>
    );

    let on_submit = props.on_form_submit.clone().map(move |on_submit| Callback::from(move |evt: SubmitEvent| {
        evt.prevent_default();
        on_submit.emit(());
    }));
    let modal = if !props.is_form {
        html!(
            <div class={modal_style}>
                {modal_content}
            </div>
        )
    } else {
        html!(
            <form class={modal_style} onsubmit={on_submit.unwrap()}>
                {modal_content}
            </form>
        )
    };


    let modal_host = if let Some(modal_host) = gloo::utils::document().get_element_by_id((*modal_id).clone().as_str()) {
        modal_host
    } else {
        let modal_host = gloo::utils::document().create_element("div").expect("Failed to create div");
        modal_host.class_list().add_1("cosmo-modal__container").expect("Should be able to add class");
        modal_host.set_id((*modal_id).clone().as_str());
        gloo::utils::body().append_child(&modal_host).expect("Failed to append child");
        modal_host
    };
    {
        let modal_host = modal_host.clone();
        use_unmount(move || modal_host.remove());
    }

    create_portal(
        modal,
        modal_host,
    )
}

#[derive(Properties, PartialEq, Clone)]
pub struct CosmoAlertProps {
    pub title: AttrValue,
    pub message: AttrValue,
    pub close_label: AttrValue,
    pub on_close: Callback<()>,
}

#[styled_component(CosmoAlert)]
pub fn alert(props: &CosmoAlertProps) -> Html {
    let on_close = props.on_close.clone();

    html!(
        <CosmoModal title={props.title.clone()} buttons={html!(<CosmoButton on_click={move |_| on_close.emit(())} label={props.close_label.clone()} />)}>
            {props.message.clone()}
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
}

#[styled_component(CosmoConfirm)]
pub fn confirm(props: &CosmoConfirmProps) -> Html {
    let on_confirm = props.on_confirm.clone();
    let on_decline = props.on_decline.clone();

    html!(
        <CosmoModal title={props.title.clone()} buttons={html!(
            <>
                <CosmoButton on_click={move |_| on_decline.emit(())} label={props.decline_label.clone()} />
                <CosmoButton on_click={move |_| on_confirm.emit(())} label={props.confirm_label.clone()} />
            </>
        )}>
            {props.message.clone()}
        </CosmoModal>
    )
}
