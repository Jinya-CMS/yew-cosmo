use yew::prelude::*;

use yew_cosmo::prelude::*;

use crate::ui::CosmoDemo;
use crate::ui::CosmoDocsPre;

#[function_component(Message)]
pub fn message() -> Html {
    html!(
        <>
            <CosmoTitle title="Message" />
            <CosmoParagraph>
                {"Cosmo provides three types of messages to display information to your user, positive, negative and information"}
            </CosmoParagraph>
            <CosmoDemo>
                <CosmoMessage message_type={CosmoMessageType::Information} header="Information" message="I am just an information, don't worry about me" />
                <CosmoMessage message_type={CosmoMessageType::Warning} header="Warning" message="I am a warning message, keep your eyes open and check before you click" />
                <CosmoMessage message_type={CosmoMessageType::Positive} header="Positive" message="I am a positive message, something worked or is a good thing to do" />
                <CosmoMessage message_type={CosmoMessageType::Negative} header="Negative" message="I am a negative message, something didn't work or is dangerous to do" />
            </CosmoDemo>
            <CosmoDocsPre>{r#"<CosmoMessage message_type={CosmoMessageType::Information} header="Information" message="I am just an information, don't worry about me" />
<CosmoMessage message_type={CosmoMessageType::Warning} header="Warning" message="I am a warning message, keep your eyes open and check before you click" />
<CosmoMessage message_type={CosmoMessageType::Positive} header="Positive" message="I am a positive message, something worked or is a good thing to do" />
<CosmoMessage message_type={CosmoMessageType::Negative} header="Negative" message="I am a negative message, something didn't work or is dangerous to do" />"#}</CosmoDocsPre>
        </>
    )
}
