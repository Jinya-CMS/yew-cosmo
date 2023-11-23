use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use color_art::Color;
use stylist::yew::{styled_component, use_style};
use web_sys::{File, HtmlElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::VChild;
use yew_hooks::use_click_away;

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
    CosmoModernSelect(VChild<CosmoModernSelect>),
}

#[allow(clippy::from_over_into)]
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
            CosmoInputGroupChildren::CosmoModernSelect(child) => child.into(),
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
    let group_style = use_style!(
        r#"
display: grid;
align-items: center;
grid-template-columns: [label] auto [input] 1fr;
grid-auto-rows: auto;
grid-auto-flow: row;
gap: var(--input-group-gap);
    "#
    );

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
    Full,
}

impl ToString for CosmoInputWidth {
    fn to_string(&self) -> String {
        match self {
            CosmoInputWidth::Auto => "auto",
            CosmoInputWidth::Small => "var(--input-width-small)",
            CosmoInputWidth::Medium => "var(--input-width-medium)",
            CosmoInputWidth::Large => "var(--input-width-large)",
            CosmoInputWidth::Full => "100%",
        }
            .to_string()
    }
}

#[hook]
fn use_input_styling(width: CosmoInputWidth) -> (Classes, Classes) {
    let label_style = use_style!(
        r#"
font-size: var(--font-size);
min-width: 10rem;
    "#
    );
    let input_style = use_style!(
        r#"
--border-indicator-color: var(--control-border-color);

transition: border-color var(--transition-duration);
min-width: ${width};
width: ${width};
height: var(--control-height);
box-sizing: border-box;
font-family: var(--font-family);
font-size: var(--font-size);
background: var(--white);
color: var(--black);
border-radius: var(--border-radius);
line-height: var(--line-height);
border: var(--input-border-width) solid var(--control-border-color);
padding: var(--input-padding-top) var(--input-padding-right) var(--input-padding-bottom)
    var(--input-padding-left);
border-bottom: var(--input-border-bottom-width) solid var(--border-indicator-color);

&:active,
&:invalid,
&:focus {
	outline: none;
	box-shadow: none;
}

&:focus {
	--border-indicator-color: var(--primary-color);
}

&:invalid {
	--border-indicator-color: var(--negative-color);
}

&:disabled {
   	--border-indicator-color: var(--disabled-color);

	cursor: not-allowed;
}
    "#,
        width = width.to_string()
    );

    (classes!(label_style), classes!(input_style))
}

#[hook]
fn use_id(id: Option<AttrValue>) -> AttrValue {
    let id_state = use_state_eq(|| id.unwrap_or(AttrValue::from(uuid::Uuid::new_v4().to_string())));

    (*id_state).clone()
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
    let id = use_id(props.id.clone());
    let oninput = use_callback(props.clone(), |evt: InputEvent, props| {
        let naive = NaiveDateTime::parse_from_str(
            evt.target_unchecked_into::<HtmlInputElement>()
                .value()
                .as_str(),
            "%FT%R",
        );

        if let Ok(naive) = naive {
            let local = Local {};
            props
                .on_input
                .emit(local.from_local_datetime(&naive).unwrap())
        }
    });

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
    let id = use_id(props.id.clone());
    let oninput = use_callback(props.clone(), |evt: InputEvent, props| {
        props.on_input.emit(
            NaiveDate::parse_from_str(
                evt.target_unchecked_into::<HtmlInputElement>()
                    .value()
                    .as_str(),
                "%F",
            )
                .unwrap_or(props.value),
        )
    });

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
    let id = use_id(props.id.clone());
    let oninput = use_callback(props.clone(), |evt: InputEvent, props| {
        props.on_input.emit(
            NaiveTime::parse_from_str(
                evt.target_unchecked_into::<HtmlInputElement>()
                    .value()
                    .as_str(),
                "%R",
            )
                .unwrap_or(props.value),
        )
    });

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
        }
            .to_string()
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
    let id = use_id(props.id.clone());
    let oninput = use_callback(props.clone(), |evt: InputEvent, props| {
        props.on_input.emit(
            evt.target_unchecked_into::<HtmlInputElement>()
                .value()
                .into(),
        )
    });

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
    let id = use_id(props.id.clone());
    let onchange = use_callback(props.clone(), |evt: Event, props| {
        if let Some(files) = evt.target_unchecked_into::<HtmlInputElement>().files() {
            if let Some(file) = files.get(0) {
                props.on_select.emit(file);
            }
        }
    });

    let (label_style, input_style) = use_input_styling(props.width.clone());
    let file_picker_style = use_style!(
        r#"
padding: 0;

&::file-selector-button,
&::-webkit-file-upload-button {
	--button-border-color: var(--control-border-color);
	--button-background: var(--white);
	--button-color: var(--black);

	border-top: 0;
	border-left: 0;
	border-bottom: 0;
	cursor: pointer;
	font-family: var(--font-family);
	font-size: var(--font-size);
	padding: var(--button-padding-top) var(--button-padding-right) var(--button-padding-bottom)
		var(--button-padding-left);
	box-sizing: border-box;
	border: none;
	border-right: var(--button-border-width) solid var(--button-border-color);
	background: var(--button-background);
	color: var(--button-color);
	line-height: var(--line-height);
	text-decoration: none;
	font-weight: normal;
	border-radius: 0;
	height: var(--control-height);
	transition:
		background-color var(--transition-duration),
		color var(--transition-duration);
}

&:disabled {
	cursor: not-allowed;
}

&:disabled::file-selector-button,
&:disabled::-webkit-file-upload-button {
	--button-background: var(--disabled-color);
	--button-color: var(--white);

	cursor: not-allowed;
}

&:not(:disabled):hover::file-selector-button,
&:not(:disabled):hover::-webkit-file-upload-button {
	--button-background: var(--control-border-color);

	outline: none;
	box-shadow: none;
}

&:not(:disabled):focus::file-selector-button,
&:not(:disabled):focus::-webkit-file-upload-button {
	--button-border-color: var(--control-border-color-dark);
}

&:not(:disabled):active::file-selector-button,
&:not(:disabled):active::-webkit-file-upload-button {
	--button-border-color: var(--control-border-color-darker);
	--button-background: var(--control-border-color-darker);
	--button-color: var(--white);
}

&:not(:disabled):active {
	border-bottom-color: var(--control-border-color-darker);
}
    "#
    );

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
    let id = use_id(props.id.clone());
    let oninput = use_callback(props.clone(), |evt: InputEvent, props| {
        props.on_input.emit(
            evt.target_unchecked_into::<HtmlInputElement>()
                .value()
                .as_str()
                .parse::<i64>()
                .unwrap_or(props.value),
        )
    });

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
    let id = use_id(props.id.clone());
    let oninput = use_callback(props.clone(), |evt: InputEvent, props| {
        props.on_input.emit(
            evt.target_unchecked_into::<HtmlInputElement>()
                .value()
                .as_str()
                .parse::<f64>()
                .unwrap_or(props.value),
        )
    });

    let (label_style, input_style) = use_input_styling(props.width.clone());
    let decimal_places = format!(
        "0.{}1",
        (0..props.decimal_places - 1)
            .map(|_| "0")
            .collect::<Vec<&str>>()
            .join("")
    );

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
    let id = use_id(props.id.clone());
    let oninput = use_callback(props.clone(), |evt: InputEvent, props| {
        props.on_input.emit(
            evt.target_unchecked_into::<HtmlTextAreaElement>()
                .value()
                .into(),
        )
    });

    let (label_style, input_style) = use_input_styling(props.width.clone());
    let textarea_label_style = use_style!(
        r#"
align-self: baseline;
    "#
    );
    let textarea_style = use_style!(
        r#"
height: unset;
    "#
    );
    let mut textarea_monospace_style = Some(use_style!(
        r#"
font-family: var(--font-family-code);
    "#
    ));
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

fn rgb2hex(color: Color) -> String {
    let r = color.red();
    let g = color.green();
    let b = color.blue();
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

#[styled_component(CosmoColorPicker)]
pub fn color_picker(props: &CosmoColorPickerProps) -> Html {
    let id = use_id(props.id.clone());
    let oninput = use_callback(props.on_input.clone(), |evt: InputEvent, on_input| {
        if let Ok(color) = Color::from_hex(
            evt.target_unchecked_into::<HtmlInputElement>()
                .value()
                .as_str(),
        ) {
            on_input.emit(color);
        }
    });

    let (label_style, input_style) = use_input_styling(props.width.clone());
    html!(
        <>
            <label class={label_style} for={id.clone()}>{props.label.clone()}</label>
            <input class={input_style} readonly={props.readonly} id={id.clone()} required={props.required} type="color" value={rgb2hex(props.value)} oninput={oninput} />
        </>
    )
}

#[hook]
fn use_radio_check_switch_style() -> Classes {
    let style = use_style!(r#"
--border-indicator-color: var(--control-border-color);

appearance: none;
margin: 0;
display: flex;
position: relative;
outline: none;
border: none;
box-shadow: none;

&:invalid {
	--border-indicator-color: var(--negative-color);

	outline: none;
	box-shadow: none;
}
"#);

    classes!(style)
}

#[hook]
fn use_radio_check_switch_group_style() -> Classes {
    let style = use_style!(r#"
grid-column: 2/3;
row-gap: var(--input-group-special-gap);
column-gap: 0;
display: grid;
align-items: center;
grid-template-columns: [label] auto [input] 1fr;
grid-auto-rows: auto;
grid-auto-flow: row;
"#);

    classes!(style)
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
    let group_style = use_radio_check_switch_group_style();
    let base_style = use_radio_check_switch_style();
    let checkbox_style = use_style!(
        r#"
&:disabled {
    --border-indicator-color: var(--disabled-color);
}

&:disabled + label {
    cursor: not-allowed;
}

&::after,
&::before {
	transition:
		border-color var(--transition-duration),
		background-color var(--transition-duration);
}

&::before {
	content: '';
	display: inline-block;
	border: var(--input-border-width) solid var(--border-indicator-color);
	height: var(--checkbox-size);
	width: var(--checkbox-size);
	margin-right: calc(var(--checkbox-size) / 2);
	border-radius: var(--border-radius);
	background: var(--white);
}

&:checked::after {
	content: '';
	position: absolute;
	display: block;
	height: var(--checkbox-mark-shortarm);
	width: var(--checkbox-mark-longarm);
	border-right: var(--checkbox-mark-stroke-width) solid var(--white);
	border-top: var(--checkbox-mark-stroke-width) solid var(--white);
	transform: rotate(135deg);
	top: calc(var(--checkbox-mark-longarm) - var(--checkbox-mark-shortarm));
	left: calc(var(--checkbox-mark-longarm) / 2);
	box-sizing: content-box;
}

&:checked::before {
	--border-indicator-color: var(--primary-color);

	background: var(--border-indicator-color);
	color: var(--white);

    @media screen and (prefers-color-scheme: dark) {
        --border-indicator-color: var(--primary-color-dark);
    }
}

&:disabled:checked::before {
	--border-indicator-color: var(--disabled-color);
}
    "#
    );

    let id = use_id(props.id.clone());
    let onclick = use_callback(props.on_check.clone(), |evt: MouseEvent, on_check_change| {
        on_check_change.emit(evt.target_unchecked_into::<HtmlInputElement>().checked())
    });

    html!(
        <div class={group_style}>
            <input type="checkbox" required={props.required} checked={props.checked} id={id.clone()} onclick={onclick} readonly={props.readonly} class={classes!(base_style, checkbox_style)} />
            <label for={id.clone()}>{props.label.clone()}</label>
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
    let group_style = use_radio_check_switch_group_style();
    let base_style = use_radio_check_switch_style();
    let switch_style = use_style!(
        r#"
&:not(:checked):invalid::after {
	--border-indicator-color: var(--negative-color);

	outline: none;
	box-shadow: none;
}

&:disabled:not(:checked)::after {
	--border-indicator-color: var(--disabled-color);
}

&:checked::before,
&:not(:checked)::after {
	--border-indicator-color: var(--primary-color);

	background: var(--border-indicator-color);
	color: var(--white);

    @media screen and (prefers-color-scheme: dark) {
		--border-indicator-color: var(--primary-color-dark);
    }
}

&:disabled:checked::before {
	--border-indicator-color: var(--disabled-color);
}

&::after,
&::before {
	content: '';
	display: inline-block;
	border-radius: var(--border-radius);
	transition: all calc(var(--transition-duration) / 3);
}

&::after {
	position: absolute;
	height: var(--switch-thumb-size);
	width: var(--switch-thumb-size);
	background: var(--border-indicator-color);
	margin-top: var(--switch-thumb-margin);
	margin-left: var(--switch-thumb-margin);
}

&::before {
	display: inline-block;
	border: var(--switch-rail-border-width) solid var(--border-indicator-color);
	height: var(--switch-rail-height);
	width: var(--switch-rail-width);
	margin-right: calc(var(--switch-rail-height) / 2);
}

&:checked::after,
&:disabled:checked::after {
	margin-left: calc((var(--switch-thumb-margin) * 3) + var(--switch-thumb-size));
	background: var(--white);
}
    "#
    );

    let id = use_id(props.id.clone());
    let onclick = use_callback(props.on_check.clone(), |evt: MouseEvent, on_check_change| {
        on_check_change.emit(evt.target_unchecked_into::<HtmlInputElement>().checked())
    });

    html!(
        <div class={group_style}>
            <input type="checkbox" required={props.required} checked={props.checked} id={id.clone()} onclick={onclick} readonly={props.readonly} class={classes!(base_style, switch_style)} />
            <label for={id.clone()}>{props.label.clone()}</label>
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
    let id = use_id(props.id.clone());
    let onchange = use_callback(props.clone(), |evt: Event, props| {
        let val = evt.target_unchecked_into::<HtmlSelectElement>().value();
        if val == *"None" {
            props.on_select.emit(None)
        } else {
            props.on_select.emit(Some(AttrValue::from(val)))
        }
    });

    let (label_style, input_style) = use_input_styling(props.width.clone());
    let select_style = use_style!(
        r#"
border-bottom: var(--input-border-width) solid var(--control-border-color);
appearance: none;
position: relative;
background-image: var(--dropdown-background);
background-repeat: no-repeat;
background-position-x: right;
background-position-y: center;
padding-right: calc(var(--input-padding-right) * 4);
border-color: var(--border-indicator-color);
    "#
    );

    html!(
        <>
            <label class={label_style} for={id.clone()}>{props.label.clone()}</label>
            <select class={classes!(input_style, select_style)} disabled={props.readonly} id={id.clone()} required={props.required} onchange={onchange}>
                {for props.items.iter().map(|(id, label)| html!(<option key={if let Some(id) = id { id.to_string() } else { uuid::Uuid::new_v4().to_string() }} selected={props.value.clone() == id.clone()} value={id.clone()}>{label.clone()}</option>))}
            </select>
        </>
    )
}

#[derive(PartialEq, Clone)]
pub struct CosmoModernSelectItem {
    pub label: AttrValue,
    pub value: AttrValue,
    pub selected: bool,
}

impl CosmoModernSelectItem {
    pub fn new(label: impl Into<AttrValue>, value: impl Into<AttrValue>, selected: bool) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            selected,
        }
    }
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoModernSelectProps {
    pub label: AttrValue,
    pub on_select: Callback<AttrValue>,
    #[prop_or_default]
    pub on_deselect: Option<Callback<AttrValue>>,
    #[prop_or_default]
    pub on_filter: Option<Callback<AttrValue>>,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or(false)]
    pub readonly: bool,
    pub items: Vec<CosmoModernSelectItem>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub width: CosmoInputWidth,
}

#[styled_component(CosmoModernSelect)]
pub fn modern_select(props: &CosmoModernSelectProps) -> Html {
    let id = use_id(props.id.clone());

    let flyout_open_state = use_state_eq(|| false);
    let flyout_up_state = use_state_eq(|| false);

    let search_state = use_state_eq(|| AttrValue::from(""));

    let is_multiple = props.on_deselect.is_some();

    let select_node = use_node_ref();

    let on_open_flyout = use_callback((flyout_open_state.clone(), flyout_up_state.clone()), |evt: MouseEvent, (flyout_open_state, flyout_up_state)| {
        let is_up = if let Some(element) = evt.target_dyn_into::<HtmlElement>() {
            gloo::utils::window()
                .inner_height()
                .expect("No window? Then this app won't work")
                .as_f64()
                .expect("This should be a number")
                - element.get_bounding_client_rect().bottom()
                < 100.0
        } else {
            false
        };
        flyout_up_state.set(is_up);
        flyout_open_state.set(true);
    });
    let on_close_flyout = use_callback(flyout_open_state.clone(), |_, state| state.set(false));
    let on_deselect = use_callback(props.on_deselect.clone(), |item: AttrValue, on_deselect| {
        if let Some(evt) = on_deselect.clone() {
            evt.emit(item)
        };
    });
    let on_select = use_callback((
                                     props.on_select.clone(),
                                     search_state.clone(),
                                     flyout_open_state.clone(),
                                     is_multiple,
                                 ), |item, (on_select, search_state, flyout_open_state, is_multiple)| {
        search_state.set("".into());
        flyout_open_state.set(*is_multiple);

        on_select.emit(item);
    });
    let on_filter = use_callback((props.on_filter.clone(), search_state.clone()), |evt: InputEvent, (on_filter, search_state)| {
        let search = AttrValue::from(evt.target_unchecked_into::<HtmlInputElement>().value());
        search_state.set(search.clone());

        if let Some(on) = on_filter.clone() {
            on.emit(search)
        }
    });

    let selected_items = props.items.iter().filter(|item| item.selected);
    let available_items = props
        .items
        .iter()
        .filter(|item| !is_multiple || !item.selected);

    use_click_away(select_node.clone(), move |_: Event| {
        on_close_flyout.emit(());
    });

    let invalid_style = use_style!(
        r#"
border-color: var(--negative-color);
    "#
    );
    let select_style = use_style!(
        r#"
border-bottom-width: var(--input-border-width) !important;
appearance: none;
position: relative;
padding-right: 0 !important;
    "#
    );
    let chip_style = use_style!(
        r#"
flex: 0 1 auto;
display: flex;
font-size: 0.75rem;
color: var(--black);
border: 0.0625rem solid var(--primary-color);
position: relative;
padding: 0 0.25rem;
white-space: nowrap;
height: calc(var(--control-height) - var(--input-padding-top) - var(--input-padding-bottom) - 0.125rem);
border-radius: var(--border-radius);
align-items: center;

@media screen and (prefers-color-scheme: dark) {
    border-color: var(--primary-color-dark);
}
    "#
    );
    let chip_close_style = use_style!(
        r#"
cursor: pointer;
text-decoration: none;
margin-left: 0.125rem;
    "#
    );
    let holder_style = use_style!(
        r#"
font-size: var(--font-size);
color: var(--primary-color);
background: var(--white);
border: none;
padding: 0;
width: 100%;
display: flex;
align-items: center;
flex-wrap: wrap;
gap: 0.25rem;
justify-content: flex-start;
background-image: var(--dropdown-background);
background-repeat: no-repeat;
background-position-x: right;
background-position-y: center;
    "#
    );
    let search_style = use_style!(
        r#"
flex: 1 1 auto;
padding: 0;
margin: 0;
border: 0;
font-size: var(--font-size);
color: var(--black);
font-family: var(--font-family);
background: none;
outline: none;
width: auto;
    "#
    );
    let flyout_style = use_style!(
        r#"
position: absolute;
display: flex;
width: 100%;
background: var(--white);
border: 0.0625rem solid var(--control-border-color);
left: -0.0625rem;
top: 26px;
max-height: 10rem;
overflow-y: auto;
flex-flow: row wrap;
z-index: 1000;
border-bottom-left-radius: var(--border-radius);
border-bottom-right-radius: var(--border-radius);
    "#
    );
    let flyout_up_style = use_style!(
        r#"
top: 0;
"#
    );
    let flyout_item_style = use_style!(
        r#"
flex: 0 0 100%;
min-width: 100%;
padding: 0.25rem 0.5rem;
color: var(--black);
font-family: var(--font-family);
box-sizing: border-box;
cursor: pointer;

&:hover {
    background: var(--primary-color);
    color: var(--white);

    @media screen and (prefers-color-scheme: dark) {
        color: var(--black);
    }
}
    "#
    );
    let (label_style, input_style) = use_input_styling(props.width.clone());

    let classes = if props.required && selected_items.clone().count() == 0 {
        classes!(input_style, select_style, invalid_style)
    } else {
        classes!(input_style, select_style)
    };

    let flyout_classes = if *flyout_up_state {
        classes!(flyout_style, flyout_up_style)
    } else {
        classes!(flyout_style)
    };

    html!(
        <>
            <label for={id.clone()} class={label_style} onclick={on_open_flyout.clone()}>{props.label.clone()}</label>
            <div ref={select_node} class={classes}>
                <div disabled={props.readonly} class={holder_style} onclick={on_open_flyout.clone()}>
                    {if is_multiple {
                        html!(
                            {for selected_items.clone().map(|item| {
                                let on_deselect = on_deselect.clone();
                                let deselect_item = item.clone();

                                html!(
                                    <div class={chip_style.clone()} key={item.value.to_string()}>
                                        {item.label.clone()}
                                        <a class={chip_close_style.clone()} onclick={move |_| on_deselect.emit(deselect_item.value.clone())}>{"×"}</a>
                                    </div>
                                )
                            })}
                        )
                    } else {
                        let selected_item = selected_items.clone().next();
                        if let Some(item) = selected_item {
                            html!(
                                <div class={search_style.clone()}>{item.label.clone()}</div>
                            )
                        } else {
                            html!()
                        }
                    }}
                    if props.on_filter.is_some() {
                        <span contenteditable="plaintext-only" id={id.clone()} class={search_style.clone()} oninput={on_filter}>{(*search_state).clone()}</span>
                    }
                </div>
                if *flyout_open_state && !available_items.clone().count() > 0 {
                    <div class={flyout_classes}>
                        {for available_items.clone().map(|item| {
                            let select_item = item.clone();
                            let on_select = on_select.clone();

                            html!(
                                <span key={item.value.to_string()} onclick={move |_| on_select.emit(select_item.value.clone())} class={flyout_item_style.clone()}>{item.label.clone()}</span>
                            )
                        })}
                    </div>
                }
            </div>
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
    let onchange = use_callback(props.on_change.clone(), |evt: MouseEvent, on_change| {
        let val = evt.target_unchecked_into::<HtmlInputElement>().value();
        on_change.emit(AttrValue::from(val))
    });

    let (label_style, _input_style) = use_input_styling(CosmoInputWidth::Full);
    let label_additional_style = use_style!("align-self: baseline;");
    let group_style = use_radio_check_switch_group_style();
    let base_style = use_radio_check_switch_style();
    let radio_style = use_style!(
        r#"
&:active,
&:focus {
	outline: none;
	border: none;
	box-shadow: none;
}

&::before,
&::after {
	content: '';
	border-radius: 50%;
	display: inline-block;
	transition:
		border-color var(--transition-duration),
		background-color var(--transition-duration);
}

&::before {
	background: var(--white);
	border: 1px solid var(--border-indicator-color);
	height: var(--radio-size);
	width: var(--radio-size);
	margin-right: calc(var(--radio-size) / 2);
}

&:checked::after {
	position: absolute;
	background: var(--primary-color);
	height: calc(var(--radio-size) / 2);
	width: calc(var(--radio-size) / 2);
	left: calc(var(--radio-size) / 4);
	top: calc(var(--radio-size) / 4);

    @media screen and (prefers-color-scheme: dark) {
		background: var(--primary-color-dark);
	}
}

&:disabled:checked::after {
	background: var(--disabled-color);
}

&:active {
	--border-indicator-color: var(--primary-color);

    @media screen and (prefers-color-scheme: dark) {
        --border-indicator-color: var(--primary-color-dark);
    }
}
    "#
    );

    let name = uuid::Uuid::new_v4().to_string();

    html!(
        <>
            <label class={classes!(label_style, label_additional_style)}>{props.label.clone()}</label>
            <div class={group_style}>
                {for props.options.iter().map(|(option, label)|{
                    let id = uuid::Uuid::new_v4().to_string();
                    let name = name.clone();
                    let radio_style = radio_style.clone();
                    let base_style = base_style.clone();
                    let on_change = onchange.clone();
                    let readonly = props.readonly;
                    let required = props.required;
                    let checked = props.value.clone() == option.clone();

                    html!(
                        <>
                            <input checked={checked} required={required} readonly={readonly} onclick={on_change.clone()} value={option.clone()} name={name.clone()} type="radio" id={id.clone()} class={classes!(base_style.clone(), radio_style.clone())} />
                            <label for={id.clone()}>{label.clone()}</label>
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
    let style = use_style!(
        r#"
border: none;
padding-left: 0;
padding-right: 0;
margin: 0;
appearance: none;
background: transparent;

&::-moz-range-track {
	min-width: var(--range-track-min-width);
	height: var(--range-track-height);
	background-color: var(--range-track-background);
	border-radius: var(--border-radius);
}

&::-webkit-slider-runnable-track {
	min-width: var(--range-track-min-width);
	height: var(--range-track-height);
	background-color: var(--range-track-background);
	border-radius: var(--border-radius);
}

&::-moz-range-thumb {
	appearance: none;
	width: var(--range-thumb-width);
	height: var(--range-thumb-height);
	border: var(--range-thumb-border-size) solid var(--range-thumb-border-color);
	border-radius: var(--range-thumb-border-radius);
	background-color: var(--range-thumb-background-color);
	cursor: var(--range-thumb-cursor);
}

&::-moz-range-thumb:hover,
&::-moz-range-thumb:focus,
&::-moz-range-thumb:active {
	--range-thumb-background-color: var(--primary-color);
}

&::-webkit-slider-thumb {
	appearance: none;
	width: var(--range-thumb-width);
	height: var(--range-thumb-height);
	border: var(--range-thumb-border-size) solid var(--range-thumb-border-color);
	border-radius: var(--range-thumb-border-radius);
	background-color: var(--range-thumb-background-color);
	cursor: var(--range-thumb-cursor);
	margin-top: calc(((var(--range-thumb-height) / 2) * -1) + (var(--range-track-height) / 2));
	box-sizing: content-box;
}

&::-webkit-slider-thumb:hover,
&::-webkit-slider-thumb:focus,
&::-webkit-slider-thumb:active {
	--range-thumb-background-color: var(--primary-color);
}

&:invalid {
	--range-track-background: var(--negative-color);
}

&:disabled,
&:disabled:hover,
&:disabled:focus,
&:disabled:active {
	--range-thumb-background-color: var(--disabled-color);
	--range-thumb-border-color: var(--disabled-color);
	--range-track-background: var(--disabled-color);
}
    "#
    );

    let id = use_id(props.id.clone());
    let oninput = use_callback(props.clone(), |evt: InputEvent, props| {
        props.on_input.emit(
            evt.target_unchecked_into::<HtmlInputElement>()
                .value()
                .as_str()
                .parse::<i64>()
                .unwrap_or(props.value),
        )
    });

    let (label_style, input_style) = use_input_styling(props.width.clone());

    html!(
        <>
            <label class={label_style} for={id.clone()}>{props.label.clone()}</label>
            <input class={classes!(input_style, style)} readonly={props.readonly} id={id.clone()} required={props.required} type="range" value={props.value.to_string()} oninput={oninput} />
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
    let fieldset_style = use_style!(
        r#"
min-width: 0;
padding: 0;
margin: 0;
border: 0;
grid-column: span 2;
    "#
    );
    let legend_style = use_style!(
        r#"
font-size: var(--input-header-font-size);
height: var(--input-header-font-size);
font-weight: var(--font-weight-light);
font-family: var(--font-family-heading);
text-transform: uppercase;
grid-column: span 2;
margin-top: var(--input-group-gap);
margin-bottom: var(--input-group-gap);
    "#
    );

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
    let on_submit = use_callback(props.clone(), |evt: SubmitEvent, props| {
        evt.prevent_default();
        props.on_submit.emit(());
    });

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
