use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use color_art::Color;
use num_traits::Num;
use stylist::yew::{styled_component, use_style};
use web_sys::{File, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::VChild;

use crate::prelude::*;

#[derive(Clone, derive_more::From, PartialEq)]
pub enum CosmoInputGroupChildren {
    CosmoDateTimePicker(VChild<CosmoDateTimePicker>),
    CosmoDatePicker(VChild<CosmoDatePicker>),
    CosmoTextBox(VChild<CosmoTextBox>),
    CosmoNumberBox(VChild<CosmoNumberBox>),
    CosmoDecimalBox(VChild<CosmoDecimalBox>),
    CosmoTextArea(VChild<CosmoTextArea>),
    CosmoCheckbox(VChild<CosmoCheckbox>),
    CosmoRadios(VChild<CosmoRadios>),
    CosmoSlider(VChild<CosmoSlider>),
    CosmoFieldset(VChild<CosmoFieldset>),
    CosmoColorPicker(VChild<CosmoColorPicker>),
    CosmoDropdown(VChild<CosmoDropdown>),
    CosmoSwitch(VChild<CosmoSwitch>),
    CosmoTimePicker(VChild<CosmoTimePicker>),
    CosmoFilePicker(VChild<CosmoFilePicker>),
}

impl Into<Html> for CosmoInputGroupChildren {
    fn into(self) -> Html {
        match self {
            CosmoInputGroupChildren::CosmoDateTimePicker(child) => child.into(),
            CosmoInputGroupChildren::CosmoDatePicker(child) => child.into(),
            CosmoInputGroupChildren::CosmoTextBox(child) => child.into(),
            CosmoInputGroupChildren::CosmoNumberBox(child) => child.into(),
            CosmoInputGroupChildren::CosmoDecimalBox(child) => child.into(),
            CosmoInputGroupChildren::CosmoTextArea(child) => child.into(),
            CosmoInputGroupChildren::CosmoCheckbox(child) => child.into(),
            CosmoInputGroupChildren::CosmoRadios(child) => child.into(),
            CosmoInputGroupChildren::CosmoSlider(child) => child.into(),
            CosmoInputGroupChildren::CosmoFieldset(child) => child.into(),
            CosmoInputGroupChildren::CosmoColorPicker(child) => child.into(),
            CosmoInputGroupChildren::CosmoDropdown(child) => child.into(),
            CosmoInputGroupChildren::CosmoSwitch(child) => child.into(),
            CosmoInputGroupChildren::CosmoTimePicker(child) => child.into(),
            CosmoInputGroupChildren::CosmoFilePicker(child) => child.into(),
        }
    }
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoInputGroupProps {
    #[prop_or_default]
    pub children: ChildrenRenderer<CosmoInputGroupChildren>,
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

#[derive(PartialEq, Clone, Default)]
pub enum CosmoInputWidth {
    Auto,
    Small,
    Medium,
    Large,
    #[default]
    Full
}

impl ToString for CosmoInputWidth {
    fn to_string(&self) -> String {
        match self {
            CosmoInputWidth::Auto => "auto",
            CosmoInputWidth::Small => "240px",
            CosmoInputWidth::Medium => "480px",
            CosmoInputWidth::Large => "720px",
            CosmoInputWidth::Full => "100%",
        }.to_string()
    }
}

#[hook]
fn use_input_styling(width: CosmoInputWidth) -> (Classes, Classes) {
    let label_style = use_style!(r#"
font-size: 16px;
margin-right: 10px;
min-width: 150px;
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
width: ${width};

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
    "#, width = width.to_string());

    (classes!(label_style), classes!(input_style))
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoDateTimePickerProps {
    pub on_input: Callback<DateTime<Local>>,
    pub value: DateTime<Local>,
    pub label: AttrValue,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or(false)]
    pub readonly: bool,
    #[prop_or_default]
    pub min: Option<DateTime<Local>>,
    #[prop_or_default]
    pub max: Option<DateTime<Local>>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub width: CosmoInputWidth,
}

#[styled_component(CosmoDateTimePicker)]
pub fn date_time_picker(props: &CosmoDateTimePickerProps) -> Html {
    let id = props.id.clone().unwrap_or(AttrValue::from(uuid::Uuid::new_v4().to_string()));
    let oninput = use_callback(|evt: InputEvent, props| {
        let naive = NaiveDateTime::parse_from_str(evt.target_unchecked_into::<HtmlInputElement>().value().as_str(), "%FT%R");

        if let Ok(naive) = naive {
            let local = Local {};
            props.on_input.emit(local.from_local_datetime(&naive).unwrap())
        }
    }, props.clone());

    let (label_style, input_style) = use_input_styling(props.width.clone());
    let min = props.min.map(|min| min.clone().format("%FT%R").to_string());
    let max = props.max.map(|max| max.clone().format("%FT%R").to_string());

    html!(
        <>
            <label class={label_style} for={id.clone()}>{props.label.clone()}</label>
            <input min={min} max={max} type="datetime-local" class={input_style} readonly={props.readonly} id={id.clone()} required={props.required} value={props.value.clone().format("%FT%R").to_string()} oninput={oninput} />
        </>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoDatePickerProps {
    pub on_input: Callback<NaiveDate>,
    pub value: NaiveDate,
    pub label: AttrValue,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or(false)]
    pub readonly: bool,
    #[prop_or_default]
    pub min: Option<NaiveDate>,
    #[prop_or_default]
    pub max: Option<NaiveDate>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub width: CosmoInputWidth,
}

#[styled_component(CosmoDatePicker)]
pub fn date_picker(props: &CosmoDatePickerProps) -> Html {
    let id = props.id.clone().unwrap_or(AttrValue::from(uuid::Uuid::new_v4().to_string()));
    let oninput = use_callback(|evt: InputEvent, props| props.on_input.emit(NaiveDate::parse_from_str(evt.target_unchecked_into::<HtmlInputElement>().value().as_str(), "%F").unwrap_or(props.value)), props.clone());

    let (label_style, input_style) = use_input_styling(props.width.clone());
    let min = props.min.map(|min| min.clone().format("%F").to_string());
    let max = props.max.map(|max| max.clone().format("%F").to_string());

    html!(
        <>
            <label class={label_style} for={id.clone()}>{props.label.clone()}</label>
            <input min={min} max={max} type="date" class={input_style} readonly={props.readonly} id={id.clone()} required={props.required} value={props.value.clone().format("%F").to_string()} oninput={oninput} />
        </>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTimePickerProps {
    pub on_input: Callback<NaiveTime>,
    pub value: NaiveTime,
    pub label: AttrValue,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or(false)]
    pub readonly: bool,
    #[prop_or_default]
    pub min: Option<NaiveTime>,
    #[prop_or_default]
    pub max: Option<NaiveTime>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub width: CosmoInputWidth,
}

#[styled_component(CosmoTimePicker)]
pub fn time_picker(props: &CosmoTimePickerProps) -> Html {
    let id = props.id.clone().unwrap_or(AttrValue::from(uuid::Uuid::new_v4().to_string()));
    let oninput = use_callback(|evt: InputEvent, props| props.on_input.emit(NaiveTime::parse_from_str(evt.target_unchecked_into::<HtmlInputElement>().value().as_str(), "%R").unwrap_or(props.value)), props.clone());

    let (label_style, input_style) = use_input_styling(props.width.clone());
    let min = props.min.map(|min| min.clone().format("%R").to_string());
    let max = props.max.map(|max| max.clone().format("%R").to_string());

    html!(
        <>
            <label class={label_style} for={id.clone()}>{props.label.clone()}</label>
            <input min={min} max={max} type="time" class={input_style} readonly={props.readonly} id={id.clone()} required={props.required} value={props.value.clone().format("%R").to_string()} oninput={oninput} />
        </>
    )
}

#[derive(PartialEq, Clone, Default)]
pub enum CosmoTextBoxType {
    Email,
    Password,
    Search,
    Tel,
    #[default]
    Text,
    Url,
}

impl ToString for CosmoTextBoxType {
    fn to_string(&self) -> String {
        match self {
            CosmoTextBoxType::Email => "email",
            CosmoTextBoxType::Password => "password",
            CosmoTextBoxType::Search => "search",
            CosmoTextBoxType::Tel => "tel",
            CosmoTextBoxType::Text => "text",
            CosmoTextBoxType::Url => "url",
        }.to_string()
    }
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTextBoxProps {
    #[prop_or(CosmoTextBoxType::Text)]
    pub input_type: CosmoTextBoxType,
    pub on_input: Callback<AttrValue>,
    pub value: AttrValue,
    pub label: AttrValue,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or(false)]
    pub readonly: bool,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub width: CosmoInputWidth,
}

#[styled_component(CosmoTextBox)]
pub fn textbox(props: &CosmoTextBoxProps) -> Html {
    let id = props.id.clone().unwrap_or(AttrValue::from(uuid::Uuid::new_v4().to_string()));
    let oninput = use_callback(|evt: InputEvent, props| props.on_input.emit(evt.target_unchecked_into::<HtmlInputElement>().value().into()), props.clone());

    let (label_style, input_style) = use_input_styling(props.width.clone());

    html!(
        <>
            <label class={label_style} for={id.clone()}>{props.label.clone()}</label>
            <input class={input_style} readonly={props.readonly} id={id.clone()} required={props.required} type={props.input_type.to_string()} value={props.value.clone()} oninput={oninput} />
        </>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoFilePickerProps {
    pub on_select: Callback<File>,
    pub label: AttrValue,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or(false)]
    pub readonly: bool,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub accept: AttrValue,
    #[prop_or_default]
    pub width: CosmoInputWidth,
}

#[styled_component(CosmoFilePicker)]
pub fn file_picker(props: &CosmoFilePickerProps) -> Html {
    let id = props.id.clone().unwrap_or(AttrValue::from(uuid::Uuid::new_v4().to_string()));
    let onchange = use_callback(|evt: Event, props| {
        if let Some(files) = evt.target_unchecked_into::<HtmlInputElement>().files() {
            if let Some(file) = files.get(0) {
                props.on_select.emit(file);
            }
        }
    }, props.clone());

    let (label_style, input_style) = use_input_styling(props.width.clone());
    let file_picker_style = use_style!(r#"
padding: 0;

&::file-selector-button,
&::-webkit-file-upload-button {
    cursor: pointer;
    font-family: var(--font-family);
    font-size: 16px;
    padding: 3px 16px;
    box-sizing: border-box;
    border: 1px solid var(--control-border-color);
    background: var(--white);
    color: var(--black);
    line-height: 19px;
    text-decoration: none;
    font-weight: normal;
    border-top: 0;
    border-left: 0;
    border-bottom: 0;
    padding-top: 4px;
}

&:disabled::file-selector-button,
&:disabled::-webkit-file-upload-button {
    cursor: not-allowed;
    border: 1px solid var(--control-border-color);
    background: var(--white);
    color: var(--disabled-color);
}

&:hover::file-selector-button,
&:hover::-webkit-file-upload-button {
    background: var(--primary-color);
    color: var(--white);
    outline: none;
    box-shadow: none;
}

&:disabled:hover::file-selector-button,
&:disabled:hover::-webkit-file-upload-button {
    background: var(--white);
    color: var(--control-border-color);
    outline: none;
    box-shadow: none;
}

&:focus::file-selector-button,
&:focus::-webkit-file-upload-button {
    border-color: var(--primary-color);
    outline: none;
    box-shadow: none;
}
    "#);

    html!(
        <>
            <label class={label_style} for={id.clone()}>{props.label.clone()}</label>
            <input class={classes!(input_style, file_picker_style)} accept={props.accept.clone()} onchange={onchange} readonly={props.readonly} id={id.clone()} required={props.required} type="file" />
        </>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoNumberBoxProps {
    pub on_input: Callback<i64>,
    pub value: i64,
    pub label: AttrValue,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or(false)]
    pub readonly: bool,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub width: CosmoInputWidth,
}

#[styled_component(CosmoNumberBox)]
pub fn number_box(props: &CosmoNumberBoxProps) -> Html {
    let id = props.id.clone().unwrap_or(AttrValue::from(uuid::Uuid::new_v4().to_string()));
    let oninput = use_callback(|evt: InputEvent, props| props.on_input.emit(i64::from_str_radix(evt.target_unchecked_into::<HtmlInputElement>().value().as_str(), 10).unwrap_or(props.value.clone())), props.clone());

    let (label_style, input_style) = use_input_styling(props.width.clone());
    html!(
        <>
            <label class={label_style} for={id.clone()}>{props.label.clone()}</label>
            <input class={input_style} readonly={props.readonly} id={id.clone()} required={props.required} type="number" value={props.value.to_string()} oninput={oninput} />
        </>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoDecimalBoxProps {
    pub on_input: Callback<f64>,
    pub value: f64,
    pub label: AttrValue,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or(false)]
    pub readonly: bool,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or(2)]
    pub decimal_places: u8,
    #[prop_or_default]
    pub width: CosmoInputWidth,
}

#[styled_component(CosmoDecimalBox)]
pub fn decimal_box(props: &CosmoDecimalBoxProps) -> Html {
    let id = props.id.clone().unwrap_or(AttrValue::from(uuid::Uuid::new_v4().to_string()));
    let oninput = use_callback(|evt: InputEvent, props| props.on_input.emit(f64::from_str_radix(evt.target_unchecked_into::<HtmlInputElement>().value().as_str(), 10).unwrap_or(props.value.clone())), props.clone());

    let (label_style, input_style) = use_input_styling(props.width.clone());
    let decimal_places = format!("0.{}1", (0..props.decimal_places - 1).map(|_| "0").collect::<Vec<&str>>().join(""));

    html!(
        <>
            <label class={label_style} for={id.clone()}>{props.label.clone()}</label>
            <input step={decimal_places} class={input_style} readonly={props.readonly} id={id.clone()} required={props.required} type="number" value={props.value.to_string()} oninput={oninput} />
        </>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTextAreaProps {
    pub on_input: Callback<AttrValue>,
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
    #[prop_or_default]
    pub width: CosmoInputWidth,
}

#[styled_component(CosmoTextArea)]
pub fn textarea(props: &CosmoTextAreaProps) -> Html {
    let id = props.id.clone().unwrap_or(AttrValue::from(uuid::Uuid::new_v4().to_string()));
    let oninput = use_callback(|evt: InputEvent, props| props.on_input.emit(evt.target_unchecked_into::<HtmlTextAreaElement>().value().into()), props.clone());

    let (label_style, input_style) = use_input_styling(props.width.clone());
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
pub struct CosmoColorPickerProps {
    pub on_input: Callback<Color>,
    pub value: Color,
    pub label: AttrValue,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or(false)]
    pub readonly: bool,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub width: CosmoInputWidth,
}

#[styled_component(CosmoColorPicker)]
pub fn color_picker(props: &CosmoColorPickerProps) -> Html {
    let id = props.id.clone().unwrap_or(AttrValue::from(uuid::Uuid::new_v4().to_string()));
    let oninput = use_callback(|evt: InputEvent, on_input| {
        if let Ok(color) = Color::from_hex(evt.target_unchecked_into::<HtmlInputElement>().value().as_str()) {
            on_input.emit(color);
        }
    }, props.on_input.clone());

    let (label_style, input_style) = use_input_styling(props.width.clone());
    html!(
        <>
            <label class={label_style} for={id.clone()}>{props.label.clone()}</label>
            <input class={input_style} readonly={props.readonly} id={id.clone()} required={props.required} type="color" value={props.value.hex()} oninput={oninput} />
        </>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoCheckboxProps {
    pub on_check: Callback<bool>,
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
    let onclick = use_callback(|evt: MouseEvent, on_check_change| on_check_change.emit(evt.target_unchecked_into::<HtmlInputElement>().checked()), props.on_check.clone());

    html!(
        <div class={group_style}>
            <input type="checkbox" required={props.required} checked={props.checked} id={id.clone()} onclick={onclick} readonly={props.readonly} class={checkbox_style} />
            <label for={id.clone()} class={label_style}>{props.label.clone()}</label>
        </div>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoSwitchProps {
    pub on_check: Callback<bool>,
    pub checked: bool,
    pub label: AttrValue,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or(false)]
    pub readonly: bool,
    #[prop_or_default]
    pub id: Option<AttrValue>,
}

#[function_component(CosmoSwitch)]
pub fn switch(props: &CosmoSwitchProps) -> Html {
    let group_style = use_style!(r#"
display: grid;
grid-template-columns: auto 1fr;
grid-template-rows: auto auto;
grid-column: 2/3;
    "#);
    let switch_style = use_style!(r#"
appearance: none;
margin: 0;

&:checked + label::after {
    margin-left: 18px;
    background: var(--white);
}

&:checked + label::before {
    background: var(--primary-color);
}
    "#);
    let label_style = use_style!(r#"
display: flex;
position: relative;

&::after {
    content: '';
    position: absolute;
    display: inline-block;
    height: 14px;
    width: 14px;
    background: var(--primary-color);
    margin-top: 2px;
    margin-left: 2px;
    transition: all 0.1s;
}

&::before {
    content: '';
    display: inline-block;
    transition: all 0.1s;
    border: 1px solid var(--control-border-color);
    height: 16px;
    width: 32px;
    margin-right: 8px;
}
    "#);

    let id = props.id.clone().unwrap_or(AttrValue::from(uuid::Uuid::new_v4().to_string()));
    let onclick = use_callback(|evt: MouseEvent, on_check_change| on_check_change.emit(evt.target_unchecked_into::<HtmlInputElement>().checked()), props.on_check.clone());

    html!(
        <div class={group_style}>
            <input type="checkbox" required={props.required} checked={props.checked} id={id.clone()} onclick={onclick} readonly={props.readonly} class={switch_style} />
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
    #[prop_or_default]
    pub width: CosmoInputWidth,
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

    let (label_style, _input_style) = use_input_styling(props.width.clone());
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
font-family: var(--font-family);
width: ${width};

&:focus {
    border: 1px solid var(--primary-color);
    outline: none;
    box-shadow: none;
}

option {
    font-family: var(--font-family);
}
    "#, width = props.width.to_string());

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
pub struct CosmoRadiosProps {
    pub on_change: Callback<AttrValue>,
    pub value: AttrValue,
    pub label: AttrValue,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or(false)]
    pub readonly: bool,
    pub options: Vec<(AttrValue, AttrValue)>,
}

#[styled_component(CosmoRadios)]
pub fn radio_group(props: &CosmoRadiosProps) -> Html {
    let onchange = use_callback(|evt: MouseEvent, on_change| {
        let val = evt.target_unchecked_into::<HtmlInputElement>().value();
        on_change.emit(AttrValue::from(val))
    }, props.on_change.clone());

    let (label_style, _input_style) = use_input_styling(CosmoInputWidth::Full);
    let label_additional_style = use_style!("align-self: baseline;");

    let group_style = use_style!(r#"
display: grid;
grid-template-columns: auto 1fr;
grid-template-rows: auto auto;
    "#);
    let radio_style = use_style!(r#"
appearance: none;
margin: 0;

+ label::before,
+ label::after {
    content: '';
    border-radius: 50%;
    display: inline-block;
}

+ label::before {
    border: 1px solid var(--control-border-color);
    height: 16px;
    width: 16px;
    margin-right: 8px;
}

&:checked + label::after {
    position: absolute;
    background: var(--primary-color);
    height: 10px;
    width: 10px;
    left: 4px;
    top: 4px;
}
    "#);
    let radio_label_style = use_style!(r#"
display: flex;
position: relative;
    "#);

    let name = uuid::Uuid::new_v4().to_string();

    html!(
        <>
            <label class={classes!(label_style, label_additional_style)}>{props.label.clone()}</label>
            <div class={group_style}>
                {for props.options.iter().map(|(option, label)|{
                    let id = uuid::Uuid::new_v4().to_string();
                    let name = name.clone();
                    let radio_style = radio_style.clone();
                    let radio_label_style = radio_label_style.clone();
                    let on_change = onchange.clone();
                    let readonly = props.readonly.clone();
                    let required = props.required.clone();
                    let checked = props.value.clone() == option.clone();

                    html!(
                        <>
                            <input checked={checked} required={required} readonly={readonly} onclick={on_change.clone()} value={option.clone()} name={name.clone()} type="radio" id={id.clone()} class={radio_style.clone()} />
                            <label for={id.clone()} class={radio_label_style.clone()}>{label.clone()}</label>
                        </>
                    )
                })}
            </div>
        </>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoSliderProps {
    pub value: i64,
    pub max: u64,
    #[prop_or(0)]
    pub min: i64,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or(false)]
    pub readonly: bool,
    #[prop_or(false)]
    pub required: bool,
    pub label: AttrValue,
    pub on_input: Callback<i64>,
    #[prop_or_default]
    pub width: CosmoInputWidth,
}

#[styled_component(CosmoSlider)]
pub fn slider(props: &CosmoSliderProps) -> Html {
    let style = use_style!(r#"
--range-border-color: #CCCCCC;
--range-thumb-color: #CCCCCC;
padding: 0;
border: none;
margin: 0;
box-sizing: border-box;
height: auto;
appearance: none;
min-width: 240px;
background: transparent;
width: ${width};

&::-moz-range-track {
    min-width: 100%;
    height: 4px;
    background-color: var(--range-border-color);
}

&::-webkit-slider-runnable-track {
    min-width: 100%;
    height: 4px;
    background-color: var(--range-border-color);
}

&::-moz-range-thumb {
    -webkit-appearance: none;
    width: 6px;
    height: 24px;
    border: 1px solid var(--primary-color);
    border-radius: 0;
    background-color: var(--range-thumb-color);
    cursor: pointer;
}

&::-webkit-slider-thumb  {
    -webkit-appearance: none;
    width: 6px;
    height: 24px;
    border: 1px solid var(--primary-color);
    border-radius: 0;
    background-color: var(--range-thumb-color);
    cursor: pointer;
    margin-top: -10px;
}

&:hover,
&:focus,
&:active {
    --range-thumb-color: var(--primary-color);
}
    "#, width = props.width.to_string());

    let id = props.id.clone().unwrap_or(AttrValue::from(uuid::Uuid::new_v4().to_string()));
    let oninput = use_callback(|evt: InputEvent, props| props.on_input.emit(i64::from_str_radix(evt.target_unchecked_into::<HtmlInputElement>().value().as_str(), 10).unwrap_or(props.value)), props.clone());

    let (label_style, _input_style) = use_input_styling(CosmoInputWidth::Full);

    html!(
        <>
            <label class={label_style} for={id.clone()}>{props.label.clone()}</label>
            <input class={style} readonly={props.readonly} id={id.clone()} required={props.required} type="range" value={props.value.to_string()} oninput={oninput} />
        </>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoFieldsetProps {
    pub title: AttrValue,
    pub children: ChildrenRenderer<CosmoInputGroupChildren>,
}

#[styled_component(CosmoFieldset)]
pub fn fieldset(props: &CosmoFieldsetProps) -> Html {
    let fieldset_style = use_style!(r#"
min-width: 0;
padding: 0;
margin: 0;
border: 0;
grid-column: span 2;
    "#);
    let legend_style = use_style!(r#"
font-size: 24px;
height: 24px;
font-weight: var(--font-weight-light);
text-transform: uppercase;
margin-top: 10px;
margin-bottom: 10px;
    "#);

    html!(
        <fieldset class={fieldset_style}>
            <legend class={legend_style}>{props.title.clone()}</legend>
            <CosmoInputGroup>
                {for props.children.iter()}
            </CosmoInputGroup>
        </fieldset>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoFormProps {
    #[prop_or_default]
    pub children: ChildrenRenderer<CosmoInputGroupChildren>,
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
