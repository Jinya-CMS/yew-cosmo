use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoInputGroupProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CosmoInputGroup)]
pub fn input_group(props: &CosmoInputGroupProps) -> Html {
    html!(
        <div class="cosmo-input__group">
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
    pub required: bool,
    #[prop_or_default]
    pub id: Option<AttrValue>,
}

#[function_component(CosmoTextBox)]
pub fn textbox(props: &CosmoTextBoxProps) -> Html {
    let id = props.id.clone().unwrap_or(AttrValue::from(uuid::Uuid::new_v4().to_string()));
    let oninput = use_callback(|evt: InputEvent, props| props.on_input.emit(evt.target_unchecked_into::<HtmlInputElement>().value().into()), props.clone());

    html!(
        <>
            <label class="cosmo-label" for={id.clone()}>{props.label.clone()}</label>
            <input class="cosmo-input" id={id.clone()} required={props.required} type={props.input_type.clone()} value={props.value.clone()} oninput={oninput} />
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

#[function_component(CosmoForm)]
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
