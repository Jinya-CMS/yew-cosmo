use yew::prelude::*;

use yew_cosmo::prelude::*;

use crate::ui::CosmoDemo;
use crate::ui::CosmoDocsPre;

#[function_component(HtmlControls)]
pub fn html() -> Html {
    let textbox_state = use_state_eq(|| AttrValue::from("I like Cosmo"));
    let numberbox_state = use_state_eq(|| 25);
    let decimalbox_state = use_state_eq(|| 25.03);
    let date_time_state = use_state_eq(Local::now);
    let date_state = use_state_eq(|| Local::now().date_naive());
    let time_state = use_state_eq(|| Local::now().time());
    let color_state = use_state_eq(|| color!(#514B57));
    let checkbox_state = use_state_eq(|| false);
    let switch_state = use_state_eq(|| false);
    let radios_state = use_state_eq(|| AttrValue::from("1"));
    let dropdown_state = use_state_eq(|| Some(AttrValue::from("1")));
    let slider_state = use_state_eq(|| 45);
    let alert_open_state = use_state_eq(|| false);
    let textarea_state = use_state_eq(|| AttrValue::from("I like Cosmo"));
    let modern_single_select_state = use_state_eq(|| AttrValue::from("1"));
    let modern_multiple_select_state = use_state_eq(|| vec![String::from("1")]);

    let on_textbox_input = use_callback(textbox_state.clone(),|value: AttrValue, state| state.set(value));
    let on_numberbox_input = use_callback(numberbox_state.clone(),|value: i64, state| state.set(value));
    let on_decimalbox_input = use_callback(decimalbox_state.clone(),|value: f64, state| state.set(value));
    let on_date_time_input = use_callback(date_time_state.clone(),|value: DateTime<Local>, state| state.set(value));
    let on_date_input = use_callback(date_state.clone(),|value: NaiveDate, state| state.set(value));
    let on_time_input = use_callback(time_state.clone(),|value: NaiveTime, state| state.set(value));
    let on_color_input = use_callback(color_state.clone(), |value: Color, state| state.set(value));
    let on_checkbox_check = use_callback(checkbox_state.clone(),|value: bool, state| state.set(value));
    let on_switch_check = use_callback(switch_state.clone(), |value: bool, state| state.set(value));
    let on_radios_change = use_callback(radios_state.clone(),|value: AttrValue, state| state.set(value));
    let on_dropdown_select = use_callback(dropdown_state.clone(),|value: Option<AttrValue>, state| state.set(value));
    let on_slider_input = use_callback(slider_state.clone(), |value: i64, state| state.set(value));
    let on_textarea_input = use_callback(textarea_state.clone(),|value: AttrValue, state| state.set(value));
    let on_modern_single_select_select = use_callback(modern_single_select_state.clone(),|value: AttrValue, state| state.set(value));
    let on_modern_multiple_select_select = use_callback(modern_multiple_select_state.clone(),|value: AttrValue, state| {
        let mut data = (**state).clone();
        data.push(value.to_string());
        state.set(data);
    });
    let on_modern_multiple_select_deselect = use_callback(modern_multiple_select_state.clone(),|value: AttrValue, state| {
        let mut data = (**state).clone();
        data.iter()
            .position(move |val| value.to_string().eq(val))
            .map(|item| data.swap_remove(item));
        state.set(data);
    });

    let on_form_submit = use_callback(alert_open_state.clone(), |_: (), state| state.set(true));
    let on_alert_close = use_callback(alert_open_state.clone(), |_: (), state| state.set(false));

    html!(
        <>
            if *alert_open_state {
                <CosmoAlert title="Form submit" message="You submitted the form successfully" close_label="Close" on_close={on_alert_close} />
            }
            <CosmoTitle title="HTML Controls" />
            <CosmoParagraph>
                {"Cosmo provides a well crafted design for your form controls, see the demos below."}
            </CosmoParagraph>
            <CosmoHeader level={CosmoHeaderLevel::H2} header="Forms" />
            <CosmoDemo>
                <CosmoForm on_submit={on_form_submit} buttons={html!(
                    <>
                        <CosmoButton label="Cancel" />
                        <CosmoButton label="Submit" is_submit={true} />
                    </>
                )}>
                    <CosmoFieldset title="Input controls">
                        <CosmoTextBox width={CosmoInputWidth::Small} value={(*textbox_state).clone()} on_input={on_textbox_input} label="Text input" />
                        <CosmoNumberBox width={CosmoInputWidth::Medium} value={*numberbox_state} on_input={on_numberbox_input} label="Numeric input" />
                        <CosmoDecimalBox width={CosmoInputWidth::Large} value={*decimalbox_state} on_input={on_decimalbox_input} label="Decimal input" />
                        <CosmoSlider width={CosmoInputWidth::Full} max={100} value={*slider_state} on_input={on_slider_input} label="Slider input" />
                        <CosmoTextArea width={CosmoInputWidth::Auto} value={(*textarea_state).clone()} on_input={on_textarea_input} label="Textarea input" />
                    </CosmoFieldset>
                    <CosmoFieldset title="Picker controls">
                        <CosmoDateTimePicker value={*date_time_state} on_input={on_date_time_input} label="Date Time picker" />
                        <CosmoDatePicker value={*date_state} on_input={on_date_input} label="Date picker" />
                        <CosmoTimePicker value={*time_state} on_input={on_time_input} label="Time picker" />
                        <CosmoColorPicker value={*color_state} on_input={on_color_input} label="Color picker" />
                        <CosmoFilePicker on_select={|_| {}} label="File picker" />
                    </CosmoFieldset>
                    <CosmoFieldset title="Option controls">
                        <CosmoCheckbox checked={*checkbox_state} on_check={on_checkbox_check} label="Checkbox" />
                        <CosmoSwitch checked={*switch_state} on_check={on_switch_check} label="Switch" />
                        <CosmoRadios value={(*radios_state).clone()} on_change={on_radios_change} options={vec![(AttrValue::from("0"), AttrValue::from("Item 1")), (AttrValue::from("1"), AttrValue::from("Item 2")), (AttrValue::from("2"), AttrValue::from("Item 3"))]} label="Radio Buttons" />
                        <CosmoDropdown value={(*dropdown_state).clone()} on_select={on_dropdown_select} items={vec![(Some(AttrValue::from("0")), AttrValue::from("Item 1")), (Some(AttrValue::from("1")), AttrValue::from("Item 2")), (Some(AttrValue::from("2")), AttrValue::from("Item 3"))]} label="Dropdown" />
                        <CosmoModernSelect on_select={on_modern_single_select_select} items={vec![
                            CosmoModernSelectItem::new("Item 1", "1", (*modern_single_select_state).clone() == *"1"),
                            CosmoModernSelectItem::new("Item 2", "2", (*modern_single_select_state).clone() == *"2"),
                            CosmoModernSelectItem::new("Item 3", "3", (*modern_single_select_state).clone() == *"3"),
                            CosmoModernSelectItem::new("Item 4", "4", (*modern_single_select_state).clone() == *"4"),
                            CosmoModernSelectItem::new("Item 5", "5", (*modern_single_select_state).clone() == *"5"),
                            CosmoModernSelectItem::new("Item 6", "6", (*modern_single_select_state).clone() == *"6"),
                            CosmoModernSelectItem::new("Item 7", "7", (*modern_single_select_state).clone() == *"7"),
                            CosmoModernSelectItem::new("Item 8", "8", (*modern_single_select_state).clone() == *"8"),
                            CosmoModernSelectItem::new("Item 9", "9", (*modern_single_select_state).clone() == *"9"),
                        ]} label="Modern single select" />
                        <CosmoModernSelect on_select={on_modern_multiple_select_select} on_deselect={on_modern_multiple_select_deselect} items={vec![
                            CosmoModernSelectItem::new("Item 1", "1", (*modern_multiple_select_state).clone().contains(&String::from("1"))),
                            CosmoModernSelectItem::new("Item 2", "2", (*modern_multiple_select_state).clone().contains(&String::from("2"))),
                            CosmoModernSelectItem::new("Item 3", "3", (*modern_multiple_select_state).clone().contains(&String::from("3"))),
                            CosmoModernSelectItem::new("Item 4", "4", (*modern_multiple_select_state).clone().contains(&String::from("4"))),
                            CosmoModernSelectItem::new("Item 5", "5", (*modern_multiple_select_state).clone().contains(&String::from("5"))),
                            CosmoModernSelectItem::new("Item 6", "6", (*modern_multiple_select_state).clone().contains(&String::from("6"))),
                            CosmoModernSelectItem::new("Item 7", "7", (*modern_multiple_select_state).clone().contains(&String::from("7"))),
                            CosmoModernSelectItem::new("Item 8", "8", (*modern_multiple_select_state).clone().contains(&String::from("8"))),
                            CosmoModernSelectItem::new("Item 9", "9", (*modern_multiple_select_state).clone().contains(&String::from("9"))),
                        ]} label="Modern multiple select" />
                    </CosmoFieldset>
                </CosmoForm>
            </CosmoDemo>
            <CosmoDocsPre>{r#"<CosmoForm on_submit={on_form_submit} buttons={html!(
    <>
        <CosmoButton label="Cancel" />
        <CosmoButton label="Submit" is_submit={true} />
    </>
)}>
    <CosmoFieldset title="Input controls">
        <CosmoTextBox width={CosmoInputWidth::Small} value={(*textbox_state).clone()} on_input={on_textbox_input} label="Text input" />
        <CosmoNumberBox width={CosmoInputWidth::Medium} value={(*numberbox_state).clone()} on_input={on_numberbox_input} label="Numeric input" />
        <CosmoDecimalBox width={CosmoInputWidth::Large} value={(*decimalbox_state).clone()} on_input={on_decimalbox_input} label="Decimal input" />
        <CosmoSlider width={CosmoInputWidth::Full} max={100} value={(*slider_state).clone()} on_input={on_slider_input} label="Slider input" />
        <CosmoTextArea width={CosmoInputWidth::Auto} value={(*textarea_state).clone()} on_input={on_textarea_input} label="Textarea input" />
    </CosmoFieldset>
    <CosmoFieldset title="Picker controls">
        <CosmoDateTimePicker value={(*date_time_state).clone()} on_input={on_date_time_input} label="Date Time picker" />
        <CosmoDatePicker value={(*date_state).clone()} on_input={on_date_input} label="Date picker" />
        <CosmoTimePicker value={(*time_state).clone()} on_input={on_time_input} label="Time picker" />
        <CosmoColorPicker value={(*color_state).clone()} on_input={on_color_input} label="Color picker" />
        <CosmoFilePicker on_select={|_| {}} label="File picker" />
    </CosmoFieldset>
    <CosmoFieldset title="Option controls">
        <CosmoCheckbox checked={*checkbox_state} on_check={on_checkbox_check} label="Checkbox" />
        <CosmoSwitch checked={*switch_state} on_check={on_switch_check} label="Switch" />
        <CosmoRadios value={(*radios_state).clone()} on_change={on_radios_change} options={vec![(AttrValue::from("0"), AttrValue::from("Item 1")), (AttrValue::from("1"), AttrValue::from("Item 2")), (AttrValue::from("2"), AttrValue::from("Item 3"))]} label="Radio Buttons" />
        <CosmoDropdown value={(*dropdown_state).clone()} on_select={on_dropdown_select} items={vec![(Some(AttrValue::from("0")), AttrValue::from("Item 1")), (Some(AttrValue::from("1")), AttrValue::from("Item 2")), (Some(AttrValue::from("2")), AttrValue::from("Item 3"))]} label="Dropdown" />
        <CosmoModernSelect on_select={on_modern_single_select_select} items={vec![
            CosmoModernSelectItem::new("Item 1", "1", (*modern_single_select_state).clone() == AttrValue::from("1")),
            CosmoModernSelectItem::new("Item 2", "2", (*modern_single_select_state).clone() == AttrValue::from("2")),
            CosmoModernSelectItem::new("Item 3", "3", (*modern_single_select_state).clone() == AttrValue::from("3")),
            CosmoModernSelectItem::new("Item 4", "4", (*modern_single_select_state).clone() == AttrValue::from("4")),
            CosmoModernSelectItem::new("Item 5", "5", (*modern_single_select_state).clone() == AttrValue::from("5")),
            CosmoModernSelectItem::new("Item 6", "6", (*modern_single_select_state).clone() == AttrValue::from("6")),
            CosmoModernSelectItem::new("Item 7", "7", (*modern_single_select_state).clone() == AttrValue::from("7")),
            CosmoModernSelectItem::new("Item 8", "8", (*modern_single_select_state).clone() == AttrValue::from("8")),
            CosmoModernSelectItem::new("Item 9", "9", (*modern_single_select_state).clone() == AttrValue::from("9")),
        ]} label="Modern single select" />
        <CosmoModernSelect on_select={on_modern_multiple_select_select} on_deselect={on_modern_multiple_select_deselect} items={vec![
            CosmoModernSelectItem::new("Item 1", "1", (*modern_multiple_select_state).clone().contains(&String::from("1"))),
            CosmoModernSelectItem::new("Item 2", "2", (*modern_multiple_select_state).clone().contains(&String::from("2"))),
            CosmoModernSelectItem::new("Item 3", "3", (*modern_multiple_select_state).clone().contains(&String::from("3"))),
            CosmoModernSelectItem::new("Item 4", "4", (*modern_multiple_select_state).clone().contains(&String::from("4"))),
            CosmoModernSelectItem::new("Item 5", "5", (*modern_multiple_select_state).clone().contains(&String::from("5"))),
            CosmoModernSelectItem::new("Item 6", "6", (*modern_multiple_select_state).clone().contains(&String::from("6"))),
            CosmoModernSelectItem::new("Item 7", "7", (*modern_multiple_select_state).clone().contains(&String::from("7"))),
            CosmoModernSelectItem::new("Item 8", "8", (*modern_multiple_select_state).clone().contains(&String::from("8"))),
            CosmoModernSelectItem::new("Item 9", "9", (*modern_multiple_select_state).clone().contains(&String::from("9"))),
        ]} label="Modern multiple select" />
    </CosmoFieldset>
</CosmoForm>"#}</CosmoDocsPre>
            <CosmoHeader level={CosmoHeaderLevel::H2} header="Buttons" />
            <CosmoParagraph>
                {"Cosmo provides two different button styles."}
            </CosmoParagraph>
            <CosmoHeader level={CosmoHeaderLevel::H3} header="Normal buttons" />
            <CosmoDemo>
                <CosmoButton label="Normal button" />
            </CosmoDemo>
            <CosmoDocsPre>{r#"<CosmoButton label="Normal button" on_click={on_click} />
    <CosmoButtonLink<Route> to={Route::Home} label="Normal button" />"#}</CosmoDocsPre>
            <CosmoHeader level={CosmoHeaderLevel::H3} header="Circular buttons" />
            <CosmoDemo>
                <CosmoCircleButton icon={IconId::LucideVideo} size={CosmoCircleButtonSize::Small} title="Small circular button" />
                <CosmoCircleButton icon={IconId::LucideVideo} title="Medium circular button" />
                <CosmoCircleButton icon={IconId::LucideVideo} size={CosmoCircleButtonSize::Large} title="Large circular button" />
            </CosmoDemo>
            <CosmoDocsPre>{r#"<CosmoCircleButton on_click={on_click} icon={IconId::LucideVideo} size={CosmoCircleButtonSize::Small} title="Small circular button" />
<CosmoCircleButton on_click={on_click} icon={IconId::LucideVideo} title="Medium circular button" />
<CosmoCircleButton on_click={on_click} icon={IconId::LucideVideo} size={CosmoCircleButtonSize::Large} title="Large circular button" />"#}</CosmoDocsPre>
            <CosmoHeader level={CosmoHeaderLevel::H2} header="Loaders" />
            <CosmoParagraph>
                {"Cosmo provides a progress bar and a progress ring, the progress bar can be easily be integrated into the bottom bar. The progress ring is perfect to indicate that you are loading data and can be easily integrated with a suspense."}
            </CosmoParagraph>
            <CosmoHeader level={CosmoHeaderLevel::H3} header="Progress bar" />
            <CosmoDemo>
                <CosmoProgressBar value={15} max={45} />
                <CosmoBr />
                <CosmoProgressBar is_indeterminate={true} />
            </CosmoDemo>
            <CosmoDocsPre>{r#"<CosmoProgressBar value={15} max={45} />
<CosmoBr />
<CosmoProgressBar is_indeterminate={true} />"#}</CosmoDocsPre>
            <CosmoHeader level={CosmoHeaderLevel::H3} header="Progress ring" />
            <CosmoDemo>
                <CosmoProgressRing />
            </CosmoDemo>
            <CosmoDocsPre>{r#"<CosmoProgressRing />"#}</CosmoDocsPre>
        </>
    )
}
