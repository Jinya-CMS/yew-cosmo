use stylist::yew::{styled_component, use_style};
use web_sys::Element;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_hooks::use_unmount;
use crate::button::CosmoButton;

#[hook]
pub fn use_modal() -> Element {
    let element = gloo::utils::document().create_element("div").expect("Failed to create div");
    gloo::utils::body().append_child(&element).expect("Failed to append child");
    element.class_list().add_1("cosmo-modal__container").expect("Should be able to add class");

    {
        let element = element.clone();
        use_unmount(move || element.remove());
    }

    element
}

#[derive(Properties, PartialEq, Clone)]
pub struct CosmoModalProps {
    #[prop_or_default]
    pub children: Children,
    pub buttons: VNode,
    pub title: AttrValue,
}

#[styled_component(CosmoModal)]
pub fn modal(props: &CosmoModalProps) -> Html {
    let modal_host = use_modal();
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

    create_portal(
        html!(
            <div class={modal_style}>
                <h1 class={modal_title_style}>{props.title.clone()}</h1>
                <div class={modal_content_style}>
                    {for props.children.iter()}
                </div>
                <div class={modal_button_bar_style}>
                    {props.buttons.clone()}
                </div>
            </div>
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
