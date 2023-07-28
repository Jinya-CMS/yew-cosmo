use yew::prelude::*;

use yew_cosmo::prelude::*;

use crate::ui::CosmoDocsPre;

#[function_component(Customize)]
pub fn customize() -> Html {
    html!(
        <>
            <CosmoTitle title="Customize" />
            <CosmoParagraph>
                {"Customizing the colors and fonts of cosmo is really easy, you only need to overwrite two CSS variables, to get your own color scheme. Apart from the two main colors there are nine other colors you can change. And one property to change the font family."}
            </CosmoParagraph>
            <CosmoParagraph>
                {"For the customized variables to take effect they need to be applied on the body element in your custom CSS file."}
            </CosmoParagraph>
            <CosmoHeader level={CosmoHeaderLevel::H2} header="Cosmo variables" />
            <CosmoDocsPre>{r#"--control-border-color: #CCCCCC;            /** The border color for input and button controls **/
--primary-color: #514B57;                   /** The primary color used for the highlights and accents **/
--white: #FFFFFF;                           /** The color that represents white, for dark mode setups this needs to be the darkest color **/
--black: #333333;                           /** The color that represents black, for dark mode setups this needs to be the lightest color **/
--menu-text-selected-color: var(--black);   /** The color that is used for the selected menu items **/
--menu-text-color: #00000040;               /** The color that is used for the default text of menu items **/
--disabled-color: var(--menu-text-color);   /** The color that is used for disabled controls **/
--code-color: #182B70;                      /** The color that is used to highlight code **/
--gradient-top-color: #EDEDEE;              /** The color of the upper part of the gradient that is above the menu and on the top border of modals **/
--gradient-bottom-color: var(--white);      /** The color of the lower part of the gradient that is above the menu and on the top border of modals **/
--modal-backdrop: #FFFFFF4D;                /** The backdrop color for modals **/

--font-family: Lato, sans-serif;            /** The font family used across the application **/"#}</CosmoDocsPre>
        </>
    )
}
