use yew::prelude::*;

use yew_cosmo::prelude::*;

use crate::ui::{CosmoDemo, CosmoDocsPre};

#[function_component(Dialog)]
pub fn dialog() -> Html {
    let alert_open_state = use_state_eq(|| false);
    let confirm_open_state = use_state_eq(|| false);
    let modal_open_state = use_state_eq(|| false);

    let alert_type_state = use_state_eq(|| CosmoAlertType::Primary);

    let open_alert = use_callback(|_, state| state.set(true), alert_open_state.clone());
    let close_alert = use_callback(|_, state| state.set(false), alert_open_state.clone());
    let open_confirm = use_callback(|_, state| state.set(true), confirm_open_state.clone());
    let close_confirm = use_callback(|_, state| state.set(false), confirm_open_state.clone());
    let open_modal = use_callback(|_, state| state.set(true), modal_open_state.clone());
    let close_modal = use_callback(|_, state| state.set(false), modal_open_state.clone());
    let on_alert_type_select = use_callback(|val: Option<AttrValue>, state| state.set(val.unwrap().into()), alert_type_state.clone());

    let textbox_state = use_state_eq(|| AttrValue::from("I like Cosmo"));
    let modern_single_select_state = use_state_eq(|| AttrValue::from("1"));

    let numberbox_state = use_state_eq(|| 25);
    let decimalbox_state = use_state_eq(|| 25.03);

    let on_textbox_input = use_callback(|value: AttrValue, state| state.set(value), textbox_state.clone());
    let on_numberbox_input = use_callback(|value: i64, state| state.set(value), numberbox_state.clone());
    let on_decimalbox_input = use_callback(|value: f64, state| state.set(value), decimalbox_state.clone());
    let on_modern_single_select_select = use_callback(|value: AttrValue, state| state.set(value), modern_single_select_state.clone());

    html!(
        <>
            <CosmoTitle title="Dialogs" />
            <CosmoParagraph>
                {"The dialog is in Cosmo Yew an option to ask your user for data or confirmation."}
            </CosmoParagraph>
            <CosmoParagraph>
                {"Cosmo provides a generic dialog which can be filled with basically any content and replacements for the browser internal confirm and alert dialogs."}
            </CosmoParagraph>
            <CosmoDemo>
                <CosmoToolbar>
                    <CosmoToolbarGroup>
                        <CosmoButton label="Open form modal" on_click={open_modal} />
                    </CosmoToolbarGroup>
                    <CosmoToolbarGroup>
                        <CosmoDropdown label="Alert type" value={(*alert_type_state).clone().to_string()} on_select={on_alert_type_select} items={vec![
                            (Some(CosmoAlertType::Primary.into()), "Primary".into()),
                            (Some(CosmoAlertType::Information.into()), "Information".into()),
                            (Some(CosmoAlertType::Warning.into()), "Warning".into()),
                            (Some(CosmoAlertType::Positive.into()), "Positive".into()),
                            (Some(CosmoAlertType::Negative.into()), "Negative".into()),
                        ]} />
                    </CosmoToolbarGroup>
                    <CosmoToolbarGroup>
                        <CosmoButton label="Open alert modal" on_click={open_alert} />
                        <CosmoButton label="Open confirm modal" on_click={open_confirm} />
                    </CosmoToolbarGroup>
                </CosmoToolbar>
                if *alert_open_state {
                    <CosmoAlert alert_type={(*alert_type_state).clone()} close_label="Close" title="I am an Alert" message="I am just a small alert modal, you can close me." on_close={close_alert} />
                }
                if *confirm_open_state {
                    <CosmoConfirm title="I am a confirm" message="I am a confirm modal, it is best to use me to ask the user for confirmation." confirm_label="Confirm" decline_label="Decline" on_confirm={close_confirm.clone()} on_decline={close_confirm} />
                }
                if *modal_open_state {
                    <CosmoModal is_form={true} title="I am a simple form" on_form_submit={close_modal.clone()} buttons={html!(
                        <>
                            <CosmoButton label="Discard changes" on_click={close_modal} />
                            <CosmoButton label="Save changes" is_submit={true} />
                        </>
                    )}>
                        <CosmoInputGroup>
                            <CosmoTextBox value={(*textbox_state).clone()} on_input={on_textbox_input} label="Text input" />
                            <CosmoNumberBox value={*numberbox_state} on_input={on_numberbox_input} label="Numeric input" />
                            <CosmoDecimalBox value={*decimalbox_state} on_input={on_decimalbox_input} label="Decimal input" />
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
                        </CosmoInputGroup>
                    </CosmoModal>
                }
            </CosmoDemo>
            <CosmoDocsPre>{r#"<>
    <CosmoToolbar>
        <CosmoToolbarGroup>
            <CosmoButton label="Open form modal" on_click={open_modal} />
        </CosmoToolbarGroup>
        <CosmoToolbarGroup>
            <CosmoButton label="Open alert modal" on_click={open_alert} />
            <CosmoButton label="Open confirm modal" on_click={open_confirm} />
        </CosmoToolbarGroup>
    </CosmoToolbar>
    if *alert_open_state {
        <CosmoAlert close_label="Close" title="I am an Alert" message="I am just a small alert modal, you can close me." on_close={close_alert} />
    }
    if *confirm_open_state {
        <CosmoConfirm title="I am a confirm" message="I am a confirm modal, it is best to use me to ask the user for confirmation." confirm_label="Confirm" decline_label="Decline" on_confirm={close_confirm.clone()} on_decline={close_confirm} />
    }
    if *modal_open_state {
        <CosmoModal is_form={true} title="I am a simple form" on_form_submit={close_modal.clone()} buttons={html!(
            <>
                <CosmoButton label="Discard changes" on_click={close_modal} />
                <CosmoButton label="Save changes" is_submit={true} />
            </>
        )}>
            <CosmoInputGroup>
                <CosmoTextBox value={(*textbox_state).clone()} on_input={on_textbox_input} label="Text input" />
                <CosmoNumberBox value={(*numberbox_state).clone()} on_input={on_numberbox_input} label="Numeric input" />
                <CosmoDecimalBox value={(*decimalbox_state).clone()} on_input={on_decimalbox_input} label="Decimal input" />
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
            </CosmoInputGroup>
        </CosmoModal>
    }
</>"#}</CosmoDocsPre>
        </>
    )
}
