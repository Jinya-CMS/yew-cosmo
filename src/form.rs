use chrono::{NaiveDateTime, Utc};
use stylist::yew::{styled_component, use_style};
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
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

#[hook]
fn use_input_styling() -> (Classes, Classes) {
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

    (classes!(label_style), classes!(input_style))
}


#[derive(PartialEq, Clone, Properties)]
pub struct CosmoDateTimePickerProps {
    pub on_input: Callback<NaiveDateTime>,
    pub value: NaiveDateTime,
    pub label: AttrValue,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or(false)]
    pub readonly: bool,
    #[prop_or_default]
    pub min: Option<NaiveDateTime>,
    #[prop_or_default]
    pub max: Option<NaiveDateTime>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
}

#[styled_component(CosmoDateTimePicker)]
pub fn datepicker(props: &CosmoDateTimePickerProps) -> Html {
    let id = props.id.clone().unwrap_or(AttrValue::from(uuid::Uuid::new_v4().to_string()));
    let oninput = use_callback(|evt: InputEvent, props| props.on_input.emit(NaiveDateTime::parse_from_str(evt.target_unchecked_into::<HtmlInputElement>().value().as_str(), "%FT%T").unwrap_or(Utc::now().naive_utc())), props.clone());

    let (label_style, input_style) = use_input_styling();
    let min = props.min.map(|min| min.clone().format("%FT%T").to_string());
    let max = props.max.map(|max| max.clone().format("%FT%T").to_string());

    html!(
        <>
            <label class={label_style} for={id.clone()}>{props.label.clone()}</label>
            <input min={min} max={max} type="datetime-local" class={input_style} readonly={props.readonly} id={id.clone()} required={props.required} value={props.value.clone().format("%FT%T").to_string()} oninput={oninput} />
        </>
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

    let (label_style, input_style) = use_input_styling();
    html!(
        <>
            <label class={label_style} for={id.clone()}>{props.label.clone()}</label>
            <input class={input_style} readonly={props.readonly} id={id.clone()} required={props.required} type={props.input_type.clone()} value={props.value.clone()} oninput={oninput} />
        </>
    )
}


#[derive(PartialEq, Clone, Properties, Debug)]
pub struct CosmoTextAreaProps {
    pub on_input: Callback<String>,
    pub value: AttrValue,
    pub label: AttrValue,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or(false)]
    pub readonly: bool,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or(10)]
    pub rows: u8,
    #[prop_or(false)]
    pub is_monospace: bool,
}

#[styled_component(CosmoTextArea)]
pub fn textarea(props: &CosmoTextAreaProps) -> Html {
    let id = props.id.clone().unwrap_or(AttrValue::from(uuid::Uuid::new_v4().to_string()));
    let oninput = use_callback(|evt: InputEvent, props| props.on_input.emit(evt.target_unchecked_into::<HtmlTextAreaElement>().value().into()), props.clone());

    let (label_style, input_style) = use_input_styling();
    let textarea_label_style = use_style!(r#"
align-self: baseline;
    "#);
    let textarea_style = use_style!(r#"
height: unset;
    "#);
    let mut textarea_monospace_style = Some(use_style!(r#"
font-family: "Source Code Pro", monospace;
    "#));
    if !props.is_monospace {
        textarea_monospace_style = None;
    }

    html!(
        <>
            <label class={classes!(label_style, textarea_label_style)} for={id.clone()}>{props.label.clone()}</label>
            <textarea rows={AttrValue::from(props.rows.to_string())} class={classes!(input_style, textarea_style, textarea_monospace_style)} readonly={props.readonly} id={id.clone()} required={props.required} oninput={oninput} value={props.value.clone()}></textarea>
        </>
    )
}


#[derive(PartialEq, Clone, Properties)]
pub struct CosmoColorBoxProps {
    pub on_input: Callback<color_art::Color>,
    pub value: color_art::Color,
    pub label: AttrValue,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or(false)]
    pub readonly: bool,
    #[prop_or_default]
    pub id: Option<AttrValue>,
}

#[styled_component(CosmoColorBox)]
pub fn colorbox(props: &CosmoColorBoxProps) -> Html {
    let id = props.id.clone().unwrap_or(AttrValue::from(uuid::Uuid::new_v4().to_string()));
    let oninput = use_callback(|evt: InputEvent, on_input| {
        if let Ok(color) = color_art::Color::from_hex(evt.target_unchecked_into::<HtmlInputElement>().value().as_str()) {
            on_input.emit(color);
        }
    }, props.on_input.clone());

    let (label_style, input_style) = use_input_styling();
    html!(
        <>
            <label class={label_style} for={id.clone()}>{props.label.clone()}</label>
            <input class={input_style} readonly={props.readonly} id={id.clone()} required={props.required} type="color" value={props.value.hex()} oninput={oninput} />
        </>
    )
}


#[derive(PartialEq, Clone, Properties)]
pub struct CosmoCheckboxProps {
    pub on_check_change: Callback<bool>,
    pub checked: bool,
    pub label: AttrValue,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or(false)]
    pub readonly: bool,
    #[prop_or_default]
    pub id: Option<AttrValue>,
}

#[function_component(CosmoCheckbox)]
pub fn checkbox(props: &CosmoCheckboxProps) -> Html {
    let group_style = use_style!(r#"
display: grid;
grid-template-columns: auto 1fr;
grid-template-rows: auto auto;
grid-column: 2/3;
    "#);
    let checkbox_style = use_style!(r#"
appearance: none;
margin: 0;

&:checked + label::after {
    content: "";
    position: absolute;
    display: block;
    height: 2px;
    width: 8px;
    border-right: 1px solid var(--white);
    border-top: 1px solid var(--white);
    transform: rotate(135deg);
    top: 7px;
    left: 5px;
}

&:checked + label::before {
    background: var(--primary-color);
    color: var(--white);
}
    "#);
    let label_style = use_style!(r#"
display: flex;
position: relative;

&::before {
    content: '';
    display: inline-block;
    border: 1px solid var(--control-border-color);
    height: 16px;
    width: 16px;
    margin-right: 8px;
}
    "#);

    let id = props.id.clone().unwrap_or(AttrValue::from(uuid::Uuid::new_v4().to_string()));
    let onclick = use_callback(|evt: MouseEvent, on_check_change| on_check_change.emit(evt.target_unchecked_into::<HtmlInputElement>().checked()), props.on_check_change.clone());

    html!(
        <div class={group_style}>
            <input type="checkbox" required={props.required} checked={props.checked} id={id.clone()} onclick={onclick} readonly={props.readonly} class={checkbox_style} />
            <label for={id.clone()} class={label_style}>{props.label.clone()}</label>
        </div>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoDropdownProps {
    pub on_select: Callback<Option<AttrValue>>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    pub label: AttrValue,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or(false)]
    pub readonly: bool,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    pub items: Vec<(Option<AttrValue>, AttrValue)>,
}

#[styled_component(CosmoDropdown)]
pub fn dropdown(props: &CosmoDropdownProps) -> Html {
    let id = props.id.clone().unwrap_or(AttrValue::from(uuid::Uuid::new_v4().to_string()));
    let onchange = use_callback(|evt: Event, props| {
        let val = evt.target_unchecked_into::<HtmlSelectElement>().value();
        if val == "None".to_string() {
            props.on_select.emit(None)
        } else {
            props.on_select.emit(Some(AttrValue::from(val)))
        }
    }, props.clone());

    let (label_style, _input_style) = use_input_styling();
    let select_style = use_style!(r#"
min-width: 240px;
font-size: 16px;
border: 1px solid var(--control-border-color);
background: var(--white);
padding: 4px 32px 4px 8px;
height: 28px;
appearance: none;
color: var(--black);
position: relative;
background-image: var(--dropdown-background);
background-repeat: no-repeat;
background-position-x: right;
background-position-y: center;

&:focus {
    border: 1px solid var(--primary-color);
    outline: none;
    box-shadow: none;
}
    "#);

    html!(
        <>
            <label class={label_style} for={id.clone()}>{props.label.clone()}</label>
            <select class={select_style} readonly={props.readonly} id={id.clone()} required={props.required} onchange={onchange}>
                {for props.items.iter().map(|(id, label)| html!(<option key={if let Some(id) = id { id.to_string() } else { uuid::Uuid::new_v4().to_string() }} selected={props.value.clone() == id.clone()} value={id.clone()}>{label.clone()}</option>))}
            </select>
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
