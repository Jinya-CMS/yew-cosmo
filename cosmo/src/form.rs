use stylist::yew::{styled_component, use_style};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoInputGroupProps {
    #[prop_or_default]
    pub children: Children,
}

#[styled_component(CosmoInputGroup)]
pub fn input_group(props: &CosmoInputGroupProps) -> Html {
    let group_style = use_style!(r#"
display: grid;
align-items: center;
grid-template-columns: [label] auto [input] 1fr;
grid-row-gap: 10px;
    "#);

    html!(
        <div class={group_style}>
            {for props.children.iter()}
        </div>
    )
}


#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTextBoxProps {
    #[prop_or(AttrValue::from("text"))]
    pub input_type: AttrValue,
    pub on_input: Callback<String>,
    pub value: AttrValue,
    pub label: AttrValue,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or(false)]
    pub readonly: bool,
    #[prop_or_default]
    pub id: Option<AttrValue>,
}

#[styled_component(CosmoTextBox)]
pub fn textbox(props: &CosmoTextBoxProps) -> Html {
    let id = props.id.clone().unwrap_or(AttrValue::from(uuid::Uuid::new_v4().to_string()));
    let oninput = use_callback(|evt: InputEvent, props| props.on_input.emit(evt.target_unchecked_into::<HtmlInputElement>().value().into()), props.clone());

    let label_style = use_style!(r#"
font-size: 16px;
margin-right: 10px;
    "#);
    let input_style = use_style!(r#"
min-width: 240px;
height: 28px;
padding: 4px 8px;
box-sizing: border-box;
font-family: var(--font-family);
font-size: 16px;
border: 1px solid var(--control-border-color);
background: var(--white);
color: var(--black);

&:focus {
    outline: none;
    box-shadow: none;
    border-color: var(--primary-color);
}

&:invalid {
    border-color: var(--negative-color);
    outline: none;
    box-shadow: none;
}
    "#);

    html!(
        <>
            <label class={label_style} for={id.clone()}>{props.label.clone()}</label>
            <input class={input_style} readonly={props.readonly} id={id.clone()} required={props.required} type={props.input_type.clone()} value={props.value.clone()} oninput={oninput} />
        </>
    )
}


#[derive(PartialEq, Clone, Properties)]
pub struct CosmoFormProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub buttons: Children,
    pub on_submit: Callback<()>,
}

#[styled_component(CosmoForm)]
pub fn form(props: &CosmoFormProps) -> Html {
    let on_submit = use_callback(|evt: SubmitEvent, props| {
        evt.prevent_default();
        props.on_submit.emit(());
    }, props.clone());

    html!(
        <form onsubmit={on_submit.clone()}>
            <CosmoInputGroup>
                {for props.children.iter()}
            </CosmoInputGroup>
            <CosmoButtonContainer>
                {for props.buttons.iter()}
            </CosmoButtonContainer>
        </form>
    )
}
