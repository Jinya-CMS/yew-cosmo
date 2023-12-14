use bounce::helmet::{Helmet, HelmetBridge};
use bounce::BounceRoot;
use std::str::FromStr;
use stylist::yew::{styled_component, use_style};
use stylist::{GlobalStyle, Style};
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::VChild;
#[cfg(feature = "with-yew-router")]
use yew_router::prelude::*;

use crate::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTitleProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub subtitle: Option<AttrValue>,
}

#[styled_component(CosmoTitle)]
pub fn title(props: &CosmoTitleProps) -> Html {
    let title_style = use_style!(
        r#"
font-weight: var(--font-weight-heading);
font-family: var(--font-family-heading);
margin: 0;
vertical-align: text-top;
font-size: var(--title-font-size);

+ .cosmo-title {
	margin-left: var(--title-gap);
	border-left: 1px solid var(--black);
	padding-left: var(--title-gap);
	box-sizing: border-box;
}

small {
    margin-left: 1rem;
}
    "#
    );

    html!(
        <h1 class={classes!(title_style, "cosmo-title")}>
            {props.title.clone()}
            if let Some(subtitle) = props.subtitle.clone() {
                <small>{subtitle}</small>
            }
        </h1>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoPageBodyProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

#[styled_component(CosmoPageBody)]
pub fn page_body(props: &CosmoPageBodyProps) -> Html {
    let page_body_content_style = use_style!(
        r#"
grid-row: content;
height: var(--page-height);
display: grid;
overflow-y: auto;
grid-template-columns: var(--page-side-spacing) [content] var(--page-width) var(--page-side-spacing);
align-content: start;

> * {
    grid-column: content;
}
    "#
    );

    html!(
        <div class={classes!(page_body_content_style, props.classes.clone())}>
            {for props.children.iter()}
        </div>
    )
}

pub type CosmoPageLayoutFormatTitle = Callback<AttrValue, AttrValue>;

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoPageLayoutProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(AttrValue::from("#19324c"))]
    pub primary_color: AttrValue,
    #[prop_or(AttrValue::from("#0f1e2e"))]
    pub primary_color_dark: AttrValue,
    #[prop_or(Callback::from(| _ | AttrValue::from("")))]
    pub format_title: CosmoPageLayoutFormatTitle,
    #[prop_or_default]
    pub default_title: AttrValue,
}

#[styled_component(CosmoPageLayout)]
pub fn page_layout(props: &CosmoPageLayoutProps) -> Html {
    let primary_color = if let Ok(color) = Color::from_str(props.primary_color.to_string().as_str())
    {
        color
    } else {
        color!(#19324c)
    };
    let primary_color_dark =
        if let Ok(color) = Color::from_str(props.primary_color_dark.to_string().as_str()) {
            color
        } else {
            color!(#0f1e2e)
        };

    let primary_color_hue = primary_color.hue();
    let primary_color_saturation = primary_color.saturation() * 100.0;
    let primary_color_lightness = primary_color.lightness() * 100.0;

    let primary_color_dark_hue = primary_color_dark.hue();
    let primary_color_dark_saturation = primary_color_dark.saturation() * 100.0;
    let primary_color_dark_lightness = primary_color_dark.lightness() * 100.0;

    let style = GlobalStyle::new(css!(r#"
*,
::before,
::after {
	box-sizing: border-box;
}

html {
	line-height: 1.15;
	-webkit-text-size-adjust: 100%;
	tab-size: 4;
}

body {
	margin: 0;
}

abbr[title] {
	text-decoration: underline dotted;
}

b,
strong {
	font-weight: bolder;
}

small {
	font-size: 80%;
}

sub,
sup {
	font-size: 75%;
	line-height: 0;
	position: relative;
	vertical-align: baseline;
}

sub {
	bottom: -0.25em;
}

sup {
	top: -0.5em;
}

::-moz-focus-inner {
	border-style: none;
	padding: 0;
}

:-moz-ui-invalid {
	box-shadow: none;
}

::-webkit-inner-spin-button,
::-webkit-outer-spin-button {
	height: auto;
}

[type='search'] {
	-webkit-appearance: textfield;
	outline-offset: -2px;
}

::-webkit-search-decoration {
	-webkit-appearance: none;
}

summary {
	display: list-item;
}

/* firefox scroll bars */
* {
	scrollbar-width: normal;
	scrollbar-color: var(--primary-color) transparent;
}

/* webkit scrollbars */

::-webkit-scrollbar {
	width: 10px;
}

::-webkit-scrollbar-thumb {
	box-shadow: inset 0 0 0 6px var(--primary-color);
	border: 2px solid transparent;
	border-radius: var(--border-radius) !important;
}

::-webkit-scrollbar-thumb:window-inactive {
	box-shadow: inset 0 0 0 6px var(--primary-color);
}

::-webkit-scrollbar-thumb:hover {
	box-shadow: inset 0 0 0 6px var(--primary-color);
}

::-webkit-scrollbar-corner {
	background: transparent;
}

::selection {
    background: var(--primary-color);
    color: var(--white);
}

:root {
	--primary-hue: ${primary_color_hue};
	--primary-saturation: ${primary_color_saturation}%;
	--primary-lightness-base: ${primary_color_lightness}%;

	--negative-hue: 2;
	--negative-saturation: 67%;
	--negative-lightness-base: 57%;

	--positive-hue: 146;
	--positive-saturation: 35%;
	--positive-lightness-base: 46%;

	--information-hue: 205;
	--information-saturation: 64%;
	--information-lightness-base: 48%;

	--warning-hue: 42;
	--warning-saturation: 54%;
	--warning-lightness-base: 54%;

	--gray-hue: 0;
	--gray-saturation: 0%;
	--gray-lightness-base: 80%;

	--light-step: 10%;
	--lighter-step: 20%;

	--dark-step: -10%;
	--darker-step: -20%;

	--white-base: 255;
	--black-base: 51;

	--border-radius: 0.25rem;

	--width-small: 15rem;
	--width-medium: calc(var(--input-width-small) * 2);
	--width-large: calc(var(--input-width-small) * 3);

	--font-size: 1rem;
	--font-weight-bold: bold;
	--font-weight-normal: normal;
	--font-weight-light: 300;
	--font-family: 'Albert Sans', sans-serif;

	--font-family-menu: Urbanist, sans-serif;
	--font-weight-menu: 200;
	--font-weight-sub-menu-active: 600;
	--font-size-main-menu: 2.5rem;
	--font-size-sub-menu: 1.25rem;
	--font-size-top-menu: 1rem;

	--font-family-modal-title: Urbanist, sans-serif;

	--font-family-heading: Urbanist, sans-serif;
	--font-weight-heading: 200;

	--font-family-code: 'Oxygen Mono', monospace;

	--line-height: 1.1875;

	--transition-duration: 0.3s;

	--control-height: 1.875rem;

	--gradient-multiplier: 4;

	--dropdown-background: url("data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzMiIgaGVpZ2h0PSI2IiB2aWV3Qm94PSIwIDAgMC4wMDAzMiA2ZS0wNSI+CiAgICA8Zz4KICAgICAgICA8cGF0aCBzdHlsZT0iZmlsbDojMDAwMDAwO3N0cm9rZS13aWR0aDowLjI2NDk5OTtzdHJva2UtbWl0ZXJsaW1pdDo0O3N0cm9rZS1kYXNoYXJyYXk6bm9uZSIKICAgICAgICAgICAgICBkPSJtIDQuODg5MDY2MSw0LjIzNDA1NTQgLTIuNDQ0NTMzLDFlLTcgTCA3Ljg1NTk1NjZlLTgsNC4yMzQwNTU0IDEuMjIyMjY2NSwyLjExNzAyNzcgMi40NDQ1MzMxLDAgMy42NjY3OTk3LDIuMTE3MDI3NiBaIgogICAgICAgICAgICAgIHRyYW5zZm9ybT0ibWF0cml4KDIuNDU0NDU2NWUtNSwwLDAsLTEuNDE3MDgxMWUtNSw5Ljk5OTk5OThlLTUsNi4wMDAwMDAxZS01KSIvPgogICAgPC9nPgo8L3N2Zz4=");

	--primary-color: hsl(var(--primary-hue) var(--primary-saturation) var(--primary-lightness-base));
	--primary-color-light: hsl(
		var(--primary-hue) var(--primary-saturation)
			calc(var(--primary-lightness-base) + var(--light-step))
	);
	--primary-color-lighter: hsl(
		var(--primary-hue) var(--primary-saturation)
			calc(var(--primary-lightness-base) + var(--lighter-step))
	);
	--primary-color-dark: hsl(
		var(--primary-hue) var(--primary-saturation)
			calc(var(--primary-lightness-base) + var(--dark-step))
	);
	--primary-color-darker: hsl(
		var(--primary-hue) var(--primary-saturation)
			calc(var(--primary-lightness-base) + var(--darker-step))
	);
	--primary-color-alpha-25: hsl(
		var(--primary-hue) var(--primary-saturation) var(--primary-lightness-base) / 25%
	);
	--primary-color-alpha-50: hsl(
		var(--primary-hue) var(--primary-saturation) var(--primary-lightness-base) / 50%
	);

	--negative-color: hsl(
		var(--negative-hue) var(--negative-saturation) var(--negative-lightness-base)
	);
	--negative-color-light: hsl(
		var(--negative-hue) var(--negative-saturation)
			calc(var(--negative-lightness-base) + var(--light-step))
	);
	--negative-color-lighter: hsl(
		var(--negative-hue) var(--negative-saturation)
			calc(var(--negative-lightness-base) + var(--lighter-step))
	);
	--negative-color-dark: hsl(
		var(--negative-hue) var(--negative-saturation)
			calc(var(--negative-lightness-base) + var(--dark-step))
	);
	--negative-color-darker: hsl(
		var(--negative-hue) var(--negative-saturation)
			calc(var(--negative-lightness-base) + var(--darker-step))
	);
	--negative-color-alpha-25: hsl(
		var(--negative-hue) var(--negative-saturation) var(--negative-lightness-base) / 25%
	);
	--negative-color-alpha-50: hsl(
		var(--negative-hue) var(--negative-saturation) var(--negative-lightness-base) / 50%
	);

	--positive-color: hsl(
		var(--positive-hue) var(--positive-saturation) var(--positive-lightness-base)
	);
	--positive-color-light: hsl(
		var(--positive-hue) var(--positive-saturation)
			calc(var(--positive-lightness-base) + var(--light-step))
	);
	--positive-color-lighter: hsl(
		var(--positive-hue) var(--positive-saturation)
			calc(var(--positive-lightness-base) + var(--lighter-step))
	);
	--positive-color-dark: hsl(
		var(--positive-hue) var(--positive-saturation)
			calc(var(--positive-lightness-base) + var(--dark-step))
	);
	--positive-color-darker: hsl(
		var(--positive-hue) var(--positive-saturation)
			calc(var(--positive-lightness-base) + var(--darker-step))
	);
	--positive-color-alpha-25: hsl(
		var(--positive-hue) var(--positive-saturation) var(--positive-lightness-base) / 25%
	);
	--positive-color-alpha-50: hsl(
		var(--positive-hue) var(--positive-saturation) var(--positive-lightness-base) / 50%
	);

	--information-color: hsl(
		var(--information-hue) var(--information-saturation) var(--information-lightness-base)
	);
	--information-color-light: hsl(
		var(--information-hue) var(--information-saturation)
			calc(var(--information-lightness-base) + var(--light-step))
	);
	--information-color-lighter: hsl(
		var(--information-hue) var(--information-saturation)
			calc(var(--information-lightness-base) + var(--lighter-step))
	);
	--information-color-dark: hsl(
		var(--information-hue) var(--information-saturation)
			calc(var(--information-lightness-base) + var(--dark-step))
	);
	--information-color-darker: hsl(
		var(--information-hue) var(--information-saturation)
			calc(var(--information-lightness-base) + var(--darker-step))
	);
	--information-color-alpha-25: hsl(
		var(--information-hue) var(--information-saturation) var(--information-lightness-base) / 25%
	);
	--information-color-alpha-50: hsl(
		var(--information-hue) var(--information-saturation) var(--information-lightness-base) / 50%
	);

	--warning-color: hsl(var(--warning-hue) var(--warning-saturation) var(--warning-lightness-base));
	--warning-color-light: hsl(
		var(--warning-hue) var(--warning-saturation)
			calc(var(--warning-lightness-base) + var(--light-step))
	);
	--warning-color-lighter: hsl(
		var(--warning-hue) var(--warning-saturation)
			calc(var(--warning-lightness-base) + var(--lighter-step))
	);
	--warning-color-dark: hsl(
		var(--warning-hue) var(--warning-saturation)
			calc(var(--warning-lightness-base) + var(--dark-step))
	);
	--warning-color-darker: hsl(
		var(--warning-hue) var(--warning-saturation)
			calc(var(--warning-lightness-base) + var(--darker-step))
	);
	--warning-color-alpha-25: hsl(
		var(--warning-hue) var(--warning-saturation) var(--warning-lightness-base) / 25%
	);
	--warning-color-alpha-50: hsl(
		var(--warning-hue) var(--warning-saturation) var(--warning-lightness-base) / 50%
	);

	--gray: hsl(var(--gray-hue), var(--gray-saturation), var(--gray-lightness-base));
	--gray-light: hsl(
		var(--gray-hue) var(--gray-saturation) calc(var(--gray-lightness-base) + var(--light-step))
	);
	--gray-lighter: hsl(
		var(--gray-hue) var(--gray-saturation) calc(var(--gray-lightness-base) + var(--lighter-step))
	);
	--gray-dark: hsl(
		var(--gray-hue) var(--gray-saturation) calc(var(--gray-lightness-base) + var(--dark-step))
	);
	--gray-darker: hsl(
		var(--gray-hue) var(--gray-saturation) calc(var(--gray-lightness-base) + var(--darker-step))
	);

	--white: rgb(var(--white-base) var(--white-base) var(--white-base));
	--black: rgb(var(--black-base) var(--black-base) var(--black-base));

	--menu-text-color: rgba(var(--black-base) var(--black-base) var(--black-base) / 25%);
	--gradient-top-color: hsl(
		var(--primary-hue) var(--primary-saturation)
			calc(var(--primary-lightness-base) * var(--gradient-multiplier))
	);
	--modal-backdrop: rgba(var(--white-base) var(--white-base) var(--white-base) / 30%);
	--modal-background: rgba(var(--white-base) var(--white-base) var(--white-base) / 90%);

	--control-border-color: var(--gray);
	--control-border-color-dark: var(--gray-dark);
	--control-border-color-darker: var(--gray-darker);
	--menu-text-selected-color: var(--black);
	--disabled-color: var(--gray-darker);
	--code-color: var(--gray-darker);
	--gradient-bottom-color: var(--white);
	--table-stripe-color: rgba(var(--black-base) var(--black-base) var(--black-base) / 20%);

	--message-padding-top: 0.5rem;
	--message-padding-bottom: 0.5rem;
	--message-padding-left: 1rem;
	--message-padding-right: 1rem;

	--message-margin-bottom: 1rem;

	--message-border-top-width: 0.25rem;

	--message-header-font-size: 1.5rem;

    --message-backdrop-filter: blur(0.5rem) saturate(90%);

	--button-padding-top: 0.25rem;
	--button-padding-bottom: 0.25rem;
	--button-padding-left: 1rem;
	--button-padding-right: 1rem;
	--button-border-width: 0.0625rem;
	--button-disabled-filter: contrast(50%);
	--button-border-color: var(--control-border-color);
	--button-background: var(--white);
	--button-color: var(--black);

	--button-circle-border-width: 0.125rem;
	--button-circle-size-small: 1.5rem;
	--button-circle-size-regular: 2rem;
	--button-circle-size-large: 3rem;
	--button-circle-background: var(--white);
	--button-circle-padding: 0.25rem;

	--button-container-margin-top: 0.75rem;
	--button-container-gap: 1rem;

	--input-border-bottom-width: 0.125rem;
	--input-border-width: 0.0625rem;
	--input-width-small: var(--width-small);
	--input-width-medium: var(--width-medium);
	--input-width-large: var(--width-large);
	--input-padding-top: 0.25rem;
	--input-padding-bottom: 0.25rem;
	--input-padding-left: 0.5rem;
	--input-padding-right: 0.5rem;

	--input-group-gap: 0.75rem;
	--input-group-special-gap: 0.25rem;

	--input-header-font-size: 1.5rem;

	--radio-size: 1rem;

	--checkbox-size: 1rem;
	--checkbox-mark-longarm: 0.5rem;
	--checkbox-mark-shortarm: 0.125rem;
	--checkbox-mark-stroke-width: 0.0625rem;

	--switch-thumb-size: 0.75rem;
	--switch-thumb-margin: calc((var(--switch-rail-height) - var(--switch-thumb-size)) / 2);
	--switch-rail-height: 1rem;
	--switch-rail-width: calc(var(--switch-rail-height) * 2);
	--switch-rail-border-width: 0.0625rem;

	--range-track-background: var(--control-border-color);
	--range-track-height: 0.25rem;
	--range-track-min-width: 100%;

	--range-thumb-color: var(--control-border-color);
	--range-thumb-border-color: var(--primary-color);
	--range-thumb-width: var(--range-track-height);
	--range-thumb-height: calc(var(--range-track-height) * 5);
	--range-thumb-border-size: var(--input-border-width);
	--range-thumb-border-radius: var(--border-radius);
	--range-thumb-background-color: var(--range-thumb-color);
	--range-thumb-cursor: pointer;

	--hr-margin-side: 2rem;
	--hr-height: 0.0625rem;

	--top-menu-height: 4rem;
	--top-menu-item-margin: 1rem;
	--top-menu-item-divider-margin-top: 0;
	--top-menu-item-divider-margin-left: var(--top-menu-item-margin);
	--top-menu-item-divider-height: 1rem;
	--top-menu-item-divider-width: 0.0625rem;

	--menu-gap: 1rem;
	--main-menu-height: var(--font-size-main-menu);
	--sub-menu-height: var(--font-size-sub-menu);

	--menu-left-touch-width: 1.5rem;

	--back-button-width: 3rem;
	--back-button-border-width: 0.25rem;
	--back-button-arrow-stroke-width: 0.25rem;
	--back-button-arrow-width: 1.25rem;
	--back-button-arrow-fin-width: 1.125rem;

	--profile-picture-size: var(--top-menu-height);

	--page-top-spacing: 2rem;
	--page-side-spacing: 10rem;
	--page-height: calc(
		100vh - var(--top-menu-height) - var(--main-menu-height) - var(--menu-gap) -
			var(--sub-menu-height) - var(--page-top-spacing) - var(--bottom-bar-spacing) -
			var(--bottom-bar-height)
	);
	--page-width: calc(100vw - var(--page-side-spacing) - var(--page-side-spacing));

	--bottom-bar-height: 4.5rem;
	--bottom-bar-spacing: 2rem;

	--list-spacing: 1rem;

	--list-items-width: 13.25rem;
	--list-items-padding-right: 1rem;
	--list-items-border-width: 0.0625rem;

	--list-item-height: 1.75rem;
	--list-item-padding-top: 0.25rem;
	--list-item-padding-bottom: 0.25rem;
	--list-item-padding-left: 0.5rem;
	--list-item-padding-right: 0.5rem;

    --modal-container-backdrop-filter: blur(0.5rem) saturate(90%);

    --modal-backdrop-filter: blur(24px) saturate(90%);
    --modal-min-width: 18rem;
    --modal-padding-top: 1.5rem;
    --modal-padding-bottom: 1.5rem;
    --modal-padding-left: 2rem;
    --modal-padding-right: 2rem;
    --modal-border-width: 0.0625rem;
    --modal-bar-width: 13rem;
    --modal-bar-height: 0.5rem;

    --modal-title-font-size: 2.25rem;
    --modal-title-margin-top: 1rem;
    --modal-title-margin-right: 0;
    --modal-title-margin-left: 0;
    --modal-title-margin-bottom: 1.25rem;

    --modal-button-bar-margin-top: 0.5rem;

	--progress-bar-gradient-width-1: 0.5rem;
	--progress-bar-gradient-width-2: 1rem;
	--progress-bar-gradient-color: var(--primary-color);
	--progress-bar-gradient-color-light: var(--primary-color-light);
	--progress-bar-width-small: var(--width-small);
	--progress-bar-width-medium: var(--width-medium);
	--progress-bar-width-large: var(--width-large);
	--progress-bar-height: 0.5rem;

	--tab-links-height: 1.25rem;
	--tab-gap: 0.75rem;

	--tab-links-gap: 1.25rem;

	--tab-link-font-size: 1.25rem;

	--table-th-padding-side: 0.5rem;
	--table-th-padding-top: 0.25rem;
	--table-th-padding-bottom: 0;
	--table-th-border-bottom-width: 0.125rem;

	--table-td-padding-side: 0.5rem;
	--table-td-padding-top: 0.25rem;
	--table-td-padding-bottom: 0.25rem;
	--table-td-border-bottom-width: 0.0625rem;

	--toolbar-gap: 1rem;

	--title-font-size: 2.25rem;
	--title-gap: 1rem;

	--h1-font-size: 2rem;
	--h2-font-size: 1.75rem;
	--h3-font-size: 1.5rem;
	--h4-font-size: 1.375rem;
	--h5-font-size: 1.25rem;
	--h6-font-size: 1rem;

	--a-color: var(--primary-color);

	--kbd-padding-top: 2px;
	--kbd-padding-bottom: 2px;
	--kbd-padding-right: 4px;
	--kbd-padding-left: 4px;

	--list-padding-left: 1rem;

	--dd-margin-bottom: 0.5rem;

	font-size: var(--font-size);
}

@media screen and (prefers-color-scheme: dark) {
	:root {
		--primary-hue: ${primary_color_dark_hue};
		--primary-saturation: ${primary_color_dark_saturation}%;
		--primary-lightness-base: ${primary_color_dark_lightness}%;

		--negative-hue: 2;
		--negative-saturation: 66%;
		--negative-lightness-base: 28%;

		--positive-hue: 146;
		--positive-saturation: 35%;
		--positive-lightness-base: 28%;

		--information-hue: 205;
		--information-saturation: 64%;
		--information-lightness-base: 28%;

		--warning-hue: 42;
		--warning-saturation: 79%;
		--warning-lightness-base: 40%;

		--gray-lightness-base: 20%;

		--light-step: -10%;
		--lighter-step: -20%;

		--dark-step: 10%;
		--darker-step: 20%;

		--white-base: 0;
		--black-base: 204;

		--gradient-multiplier: 0.25;

		--dropdown-background: url("data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzMiIgaGVpZ2h0PSI2IiB2aWV3Qm94PSIwIDAgMC4wMDAzMiA2ZS0wNSI+CiAgICA8Zz4KICAgICAgICA8cGF0aCBzdHlsZT0iZmlsbDojMWE0Zjc1O3N0cm9rZS13aWR0aDowLjI2NDk5OTtzdHJva2UtbWl0ZXJsaW1pdDo0O3N0cm9rZS1kYXNoYXJyYXk6bm9uZSIKICAgICAgICAgICAgICBkPSJtIDQuODg5MDY2MSw0LjIzNDA1NTQgLTIuNDQ0NTMzLDFlLTcgTCA3Ljg1NTk1NjZlLTgsNC4yMzQwNTU0IDEuMjIyMjY2NSwyLjExNzAyNzcgMi40NDQ1MzMxLDAgMy42NjY3OTk3LDIuMTE3MDI3NiBaIgogICAgICAgICAgICAgIHRyYW5zZm9ybT0ibWF0cml4KDIuNDU0NDU2NWUtNSwwLDAsLTEuNDE3MDgxMWUtNSw5Ljk5OTk5OThlLTUsNi4wMDAwMDAxZS01KSIvPgogICAgPC9nPgo8L3N2Zz4=");

		background: var(--white);
		color: var(--black);
	}
}

.is--light {
	--primary-hue: ${primary_color_hue};
	--primary-saturation: ${primary_color_saturation}%;
	--primary-lightness-base: ${primary_color_lightness}%;

	--negative-hue: 2;
	--negative-saturation: 67%;
	--negative-lightness-base: 57%;

	--positive-hue: 146;
	--positive-saturation: 35%;
	--positive-lightness-base: 46%;

	--information-hue: 205;
	--information-saturation: 64%;
	--information-lightness-base: 48%;

	--warning-hue: 42;
	--warning-saturation: 54%;
	--warning-lightness-base: 54%;

	--gray-lightness-base: 80%;

	--light-step: 10%;
	--lighter-step: 20%;

	--dark-step: -10%;
	--darker-step: -20%;

	--white-base: 255;
	--black-base: 51;

	--gradient-multiplier: 4;

	--dropdown-background: url("data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzMiIgaGVpZ2h0PSI2IiB2aWV3Qm94PSIwIDAgMC4wMDAzMiA2ZS0wNSI+CiAgICA8Zz4KICAgICAgICA8cGF0aCBzdHlsZT0iZmlsbDojMDAwMDAwO3N0cm9rZS13aWR0aDowLjI2NDk5OTtzdHJva2UtbWl0ZXJsaW1pdDo0O3N0cm9rZS1kYXNoYXJyYXk6bm9uZSIKICAgICAgICAgICAgICBkPSJtIDQuODg5MDY2MSw0LjIzNDA1NTQgLTIuNDQ0NTMzLDFlLTcgTCA3Ljg1NTk1NjZlLTgsNC4yMzQwNTU0IDEuMjIyMjY2NSwyLjExNzAyNzcgMi40NDQ1MzMxLDAgMy42NjY3OTk3LDIuMTE3MDI3NiBaIgogICAgICAgICAgICAgIHRyYW5zZm9ybT0ibWF0cml4KDIuNDU0NDU2NWUtNSwwLDAsLTEuNDE3MDgxMWUtNSw5Ljk5OTk5OThlLTUsNi4wMDAwMDAxZS01KSIvPgogICAgPC9nPgo8L3N2Zz4=");

	--primary-color: hsl(var(--primary-hue) var(--primary-saturation) var(--primary-lightness-base));
	--primary-color-light: hsl(
		var(--primary-hue) var(--primary-saturation)
			calc(var(--primary-lightness-base) + var(--light-step))
	);
	--primary-color-lighter: hsl(
		var(--primary-hue) var(--primary-saturation)
			calc(var(--primary-lightness-base) + var(--lighter-step))
	);
	--primary-color-dark: hsl(
		var(--primary-hue) var(--primary-saturation)
			calc(var(--primary-lightness-base) + var(--dark-step))
	);
	--primary-color-darker: hsl(
		var(--primary-hue) var(--primary-saturation)
			calc(var(--primary-lightness-base) + var(--darker-step))
	);
	--primary-color-alpha-25: hsl(
		var(--primary-hue) var(--primary-saturation) var(--primary-lightness-base) / 25%
	);
	--primary-color-alpha-50: hsl(
		var(--primary-hue) var(--primary-saturation) var(--primary-lightness-base) / 50%
	);

	--negative-color: hsl(
		var(--negative-hue) var(--negative-saturation) var(--negative-lightness-base)
	);
	--negative-color-light: hsl(
		var(--negative-hue) var(--negative-saturation)
			calc(var(--negative-lightness-base) + var(--light-step))
	);
	--negative-color-lighter: hsl(
		var(--negative-hue) var(--negative-saturation)
			calc(var(--negative-lightness-base) + var(--lighter-step))
	);
	--negative-color-dark: hsl(
		var(--negative-hue) var(--negative-saturation)
			calc(var(--negative-lightness-base) + var(--dark-step))
	);
	--negative-color-darker: hsl(
		var(--negative-hue) var(--negative-saturation)
			calc(var(--negative-lightness-base) + var(--darker-step))
	);
	--negative-color-alpha-25: hsl(
		var(--negative-hue) var(--negative-saturation) var(--negative-lightness-base) / 25%
	);
	--negative-color-alpha-50: hsl(
		var(--negative-hue) var(--negative-saturation) var(--negative-lightness-base) / 50%
	);

	--positive-color: hsl(
		var(--positive-hue) var(--positive-saturation) var(--positive-lightness-base)
	);
	--positive-color-light: hsl(
		var(--positive-hue) var(--positive-saturation)
			calc(var(--positive-lightness-base) + var(--light-step))
	);
	--positive-color-lighter: hsl(
		var(--positive-hue) var(--positive-saturation)
			calc(var(--positive-lightness-base) + var(--lighter-step))
	);
	--positive-color-dark: hsl(
		var(--positive-hue) var(--positive-saturation)
			calc(var(--positive-lightness-base) + var(--dark-step))
	);
	--positive-color-darker: hsl(
		var(--positive-hue) var(--positive-saturation)
			calc(var(--positive-lightness-base) + var(--darker-step))
	);
	--positive-color-alpha-25: hsl(
		var(--positive-hue) var(--positive-saturation) var(--positive-lightness-base) / 25%
	);
	--positive-color-alpha-50: hsl(
		var(--positive-hue) var(--positive-saturation) var(--positive-lightness-base) / 50%
	);

	--information-color: hsl(
		var(--information-hue) var(--information-saturation) var(--information-lightness-base)
	);
	--information-color-light: hsl(
		var(--information-hue) var(--information-saturation)
			calc(var(--information-lightness-base) + var(--light-step))
	);
	--information-color-lighter: hsl(
		var(--information-hue) var(--information-saturation)
			calc(var(--information-lightness-base) + var(--lighter-step))
	);
	--information-color-dark: hsl(
		var(--information-hue) var(--information-saturation)
			calc(var(--information-lightness-base) + var(--dark-step))
	);
	--information-color-darker: hsl(
		var(--information-hue) var(--information-saturation)
			calc(var(--information-lightness-base) + var(--darker-step))
	);
	--information-color-alpha-25: hsl(
		var(--information-hue) var(--information-saturation) var(--information-lightness-base) / 25%
	);
	--information-color-alpha-50: hsl(
		var(--information-hue) var(--information-saturation) var(--information-lightness-base) / 50%
	);

	--warning-color: hsl(var(--warning-hue) var(--warning-saturation) var(--warning-lightness-base));
	--warning-color-light: hsl(
		var(--warning-hue) var(--warning-saturation)
			calc(var(--warning-lightness-base) + var(--light-step))
	);
	--warning-color-lighter: hsl(
		var(--warning-hue) var(--warning-saturation)
			calc(var(--warning-lightness-base) + var(--lighter-step))
	);
	--warning-color-dark: hsl(
		var(--warning-hue) var(--warning-saturation)
			calc(var(--warning-lightness-base) + var(--dark-step))
	);
	--warning-color-darker: hsl(
		var(--warning-hue) var(--warning-saturation)
			calc(var(--warning-lightness-base) + var(--darker-step))
	);
	--warning-color-alpha-25: hsl(
		var(--warning-hue) var(--warning-saturation) var(--warning-lightness-base) / 25%
	);
	--warning-color-alpha-50: hsl(
		var(--warning-hue) var(--warning-saturation) var(--warning-lightness-base) / 50%
	);

	--gray: hsl(var(--gray-hue), var(--gray-saturation), var(--gray-lightness-base));
	--gray-light: hsl(
		var(--gray-hue) var(--gray-saturation) calc(var(--gray-lightness-base) + var(--light-step))
	);
	--gray-lighter: hsl(
		var(--gray-hue) var(--gray-saturation) calc(var(--gray-lightness-base) + var(--lighter-step))
	);
	--gray-dark: hsl(
		var(--gray-hue) var(--gray-saturation) calc(var(--gray-lightness-base) + var(--dark-step))
	);
	--gray-darker: hsl(
		var(--gray-hue) var(--gray-saturation) calc(var(--gray-lightness-base) + var(--darker-step))
	);

	--white: rgb(var(--white-base) var(--white-base) var(--white-base));
	--black: rgb(var(--black-base) var(--black-base) var(--black-base));

	--menu-text-color: rgba(var(--black-base) var(--black-base) var(--black-base) / 25%);
	--gradient-top-color: hsl(
		var(--primary-hue) var(--primary-saturation)
			calc(var(--primary-lightness-base) * var(--gradient-multiplier))
	);
	--gradient-bottom-color: var(--white);
	--modal-backdrop: rgba(var(--white-base) var(--white-base) var(--white-base) / 30%);
	--modal-background: rgba(var(--white-base) var(--white-base) var(--white-base) / 90%);
	--a-color: var(--primary-color);
	--control-border-color: var(--gray);
	--control-border-color-dark: var(--gray-dark);
	--control-border-color-darker: var(--gray-darker);
	--range-thumb-background-color: var(--range-thumb-color);
	--range-thumb-color: var(--control-border-color);
	--range-thumb-border-color: var(--primary-color);
	--range-track-background: var(--control-border-color);
	--button-circle-background: var(--white);
	--button-border-color: var(--control-border-color);
	--button-background: var(--white);
	--button-color: var(--black);
	--progress-bar-gradient-color: var(--primary-color);
	--progress-bar-gradient-color-light: var(--primary-color-light);

	background: var(--white);
	color: var(--black);
}

.is--dark {
	--primary-hue: ${primary_color_hue};
	--primary-saturation: ${primary_color_dark_saturation}%;
	--primary-lightness-base: ${primary_color_dark_lightness}%;

	--negative-hue: 2;
	--negative-saturation: 66%;
	--negative-lightness-base: 28%;

	--positive-hue: 146;
	--positive-saturation: 35%;
	--positive-lightness-base: 28%;

	--information-hue: 205;
	--information-saturation: 64%;
	--information-lightness-base: 28%;

	--warning-hue: 42;
	--warning-saturation: 79%;
	--warning-lightness-base: 40%;

	--gray-lightness-base: 20%;

	--light-step: -10%;
	--lighter-step: -20%;

	--dark-step: 10%;
	--darker-step: 20%;

	--white-base: 0;
	--black-base: 204;

	--gradient-multiplier: 0.25;

	--dropdown-background: url("data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzMiIgaGVpZ2h0PSI2IiB2aWV3Qm94PSIwIDAgMC4wMDAzMiA2ZS0wNSI+CiAgICA8Zz4KICAgICAgICA8cGF0aCBzdHlsZT0iZmlsbDojMWE0Zjc1O3N0cm9rZS13aWR0aDowLjI2NDk5OTtzdHJva2UtbWl0ZXJsaW1pdDo0O3N0cm9rZS1kYXNoYXJyYXk6bm9uZSIKICAgICAgICAgICAgICBkPSJtIDQuODg5MDY2MSw0LjIzNDA1NTQgLTIuNDQ0NTMzLDFlLTcgTCA3Ljg1NTk1NjZlLTgsNC4yMzQwNTU0IDEuMjIyMjY2NSwyLjExNzAyNzcgMi40NDQ1MzMxLDAgMy42NjY3OTk3LDIuMTE3MDI3NiBaIgogICAgICAgICAgICAgIHRyYW5zZm9ybT0ibWF0cml4KDIuNDU0NDU2NWUtNSwwLDAsLTEuNDE3MDgxMWUtNSw5Ljk5OTk5OThlLTUsNi4wMDAwMDAxZS01KSIvPgogICAgPC9nPgo8L3N2Zz4=");

	--primary-color: hsl(var(--primary-hue) var(--primary-saturation) var(--primary-lightness-base));
	--primary-color-light: hsl(
		var(--primary-hue) var(--primary-saturation)
			calc(var(--primary-lightness-base) + var(--light-step))
	);
	--primary-color-lighter: hsl(
		var(--primary-hue) var(--primary-saturation)
			calc(var(--primary-lightness-base) + var(--lighter-step))
	);
	--primary-color-dark: hsl(
		var(--primary-hue) var(--primary-saturation)
			calc(var(--primary-lightness-base) + var(--dark-step))
	);
	--primary-color-darker: hsl(
		var(--primary-hue) var(--primary-saturation)
			calc(var(--primary-lightness-base) + var(--darker-step))
	);
	--primary-color-alpha-25: hsl(
		var(--primary-hue) var(--primary-saturation) var(--primary-lightness-base) / 25%
	);
	--primary-color-alpha-50: hsl(
		var(--primary-hue) var(--primary-saturation) var(--primary-lightness-base) / 50%
	);

	--negative-color: hsl(
		var(--negative-hue) var(--negative-saturation) var(--negative-lightness-base)
	);
	--negative-color-light: hsl(
		var(--negative-hue) var(--negative-saturation)
			calc(var(--negative-lightness-base) + var(--light-step))
	);
	--negative-color-lighter: hsl(
		var(--negative-hue) var(--negative-saturation)
			calc(var(--negative-lightness-base) + var(--lighter-step))
	);
	--negative-color-dark: hsl(
		var(--negative-hue) var(--negative-saturation)
			calc(var(--negative-lightness-base) + var(--dark-step))
	);
	--negative-color-darker: hsl(
		var(--negative-hue) var(--negative-saturation)
			calc(var(--negative-lightness-base) + var(--darker-step))
	);
	--negative-color-alpha-25: hsl(
		var(--negative-hue) var(--negative-saturation) var(--negative-lightness-base) / 25%
	);
	--negative-color-alpha-50: hsl(
		var(--negative-hue) var(--negative-saturation) var(--negative-lightness-base) / 50%
	);

	--positive-color: hsl(
		var(--positive-hue) var(--positive-saturation) var(--positive-lightness-base)
	);
	--positive-color-light: hsl(
		var(--positive-hue) var(--positive-saturation)
			calc(var(--positive-lightness-base) + var(--light-step))
	);
	--positive-color-lighter: hsl(
		var(--positive-hue) var(--positive-saturation)
			calc(var(--positive-lightness-base) + var(--lighter-step))
	);
	--positive-color-dark: hsl(
		var(--positive-hue) var(--positive-saturation)
			calc(var(--positive-lightness-base) + var(--dark-step))
	);
	--positive-color-darker: hsl(
		var(--positive-hue) var(--positive-saturation)
			calc(var(--positive-lightness-base) + var(--darker-step))
	);
	--positive-color-alpha-25: hsl(
		var(--positive-hue) var(--positive-saturation) var(--positive-lightness-base) / 25%
	);
	--positive-color-alpha-50: hsl(
		var(--positive-hue) var(--positive-saturation) var(--positive-lightness-base) / 50%
	);

	--information-color: hsl(
		var(--information-hue) var(--information-saturation) var(--information-lightness-base)
	);
	--information-color-light: hsl(
		var(--information-hue) var(--information-saturation)
			calc(var(--information-lightness-base) + var(--light-step))
	);
	--information-color-lighter: hsl(
		var(--information-hue) var(--information-saturation)
			calc(var(--information-lightness-base) + var(--lighter-step))
	);
	--information-color-dark: hsl(
		var(--information-hue) var(--information-saturation)
			calc(var(--information-lightness-base) + var(--dark-step))
	);
	--information-color-darker: hsl(
		var(--information-hue) var(--information-saturation)
			calc(var(--information-lightness-base) + var(--darker-step))
	);
	--information-color-alpha-25: hsl(
		var(--information-hue) var(--information-saturation) var(--information-lightness-base) / 25%
	);
	--information-color-alpha-50: hsl(
		var(--information-hue) var(--information-saturation) var(--information-lightness-base) / 50%
	);

	--warning-color: hsl(var(--warning-hue) var(--warning-saturation) var(--warning-lightness-base));
	--warning-color-light: hsl(
		var(--warning-hue) var(--warning-saturation)
			calc(var(--warning-lightness-base) + var(--light-step))
	);
	--warning-color-lighter: hsl(
		var(--warning-hue) var(--warning-saturation)
			calc(var(--warning-lightness-base) + var(--lighter-step))
	);
	--warning-color-dark: hsl(
		var(--warning-hue) var(--warning-saturation)
			calc(var(--warning-lightness-base) + var(--dark-step))
	);
	--warning-color-darker: hsl(
		var(--warning-hue) var(--warning-saturation)
			calc(var(--warning-lightness-base) + var(--darker-step))
	);
	--warning-color-alpha-25: hsl(
		var(--warning-hue) var(--warning-saturation) var(--warning-lightness-base) / 25%
	);
	--warning-color-alpha-50: hsl(
		var(--warning-hue) var(--warning-saturation) var(--warning-lightness-base) / 50%
	);

	--gray: hsl(var(--gray-hue), var(--gray-saturation), var(--gray-lightness-base));
	--gray-light: hsl(
		var(--gray-hue) var(--gray-saturation) calc(var(--gray-lightness-base) + var(--light-step))
	);
	--gray-lighter: hsl(
		var(--gray-hue) var(--gray-saturation) calc(var(--gray-lightness-base) + var(--lighter-step))
	);
	--gray-dark: hsl(
		var(--gray-hue) var(--gray-saturation) calc(var(--gray-lightness-base) + var(--dark-step))
	);
	--gray-darker: hsl(
		var(--gray-hue) var(--gray-saturation) calc(var(--gray-lightness-base) + var(--darker-step))
	);

	--white: rgb(var(--white-base) var(--white-base) var(--white-base));
	--black: rgb(var(--black-base) var(--black-base) var(--black-base));

	--menu-text-color: rgba(var(--black-base) var(--black-base) var(--black-base) / 25%);
	--gradient-top-color: hsl(
		var(--primary-hue) var(--primary-saturation)
			calc(var(--primary-lightness-base) * var(--gradient-multiplier))
	);
	--gradient-bottom-color: var(--white);
	--modal-backdrop: rgba(var(--white-base) var(--white-base) var(--white-base) / 30%);
	--modal-background: rgba(var(--white-base) var(--white-base) var(--white-base) / 90%);

    --a-color: var(--primary-color-darker);
	--control-border-color: var(--gray);
	--control-border-color-dark: var(--gray-dark);
	--control-border-color-darker: var(--gray-darker);
    --range-thumb-background-color: var(--range-thumb-color);
    --range-thumb-color: var(--control-border-color);
    --range-thumb-border-color: var(--primary-color);
    --range-track-background: var(--control-border-color);
    --button-circle-background: var(--white);
    --button-border-color: var(--control-border-color);
    --button-background: var(--white);
    --button-color: var(--black);
    --progress-bar-gradient-color: var(--primary-color);
    --progress-bar-gradient-color-light: var(--primary-color-light);

	background: var(--white);
	color: var(--black);
}

body {
	font-family: var(--font-family);
	margin: 0;
	padding: 0;
	background: var(--white);
	color: var(--black);
	font-size: var(--font-size);
}
    "#,
        primary_color_hue = primary_color_hue,
        primary_color_saturation = primary_color_saturation,
        primary_color_lightness = primary_color_lightness,
        primary_color_dark_hue = primary_color_dark_hue,
        primary_color_dark_saturation = primary_color_dark_saturation,
        primary_color_dark_lightness = primary_color_dark_lightness,
    )).expect("Should insert global styles");

    let page_layout_style = use_style!(
        r#"
display: grid;
grid-template-rows:
    [top-menu] var(--top-menu-height) [main-menu] calc(
        var(--main-menu-height) + var(--menu-gap) + var(--sub-menu-height)
    )
    [top-spacing] var(--page-top-spacing) [content] var(--page-height) [bottom-spacing] var(--bottom-bar-spacing) [bottom-bar] var(
        --bottom-bar-height
    );
    "#
    );

    html!(
        <BounceRoot>
            <HelmetBridge default_title={props.default_title.clone()} format_title={props.format_title.clone()} />
            <Helmet>
                <link href="https://fonts.jinya.de/css2?family=Albert Sans:ital,wght@0,100%3B1,100%3B0,300%3B1,300%3B0,400%3B1,400%3B0,700%3B1,700%3B0,900%3B1,900" rel="stylesheet" type="text/css" />
                <link href="https://fonts.jinya.de/css2?family=Urbanist:ital,wght@0,100%3B1,100%3B0,300%3B1,300%3B0,400%3B1,400%3B0,700%3B1,700%3B0,900%3B1,900" rel="stylesheet" type="text/css" />
                <link href="https://fonts.jinya.de/css2?family=Oxygen Mono" rel="stylesheet" type="text/css" />
                <style>
                    {style.get_style_str()}
                </style>
            </Helmet>
            <div class={page_layout_style}>
                {for props.children.iter()}
            </div>
        </BounceRoot>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTopBarProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(false)]
    pub has_right_item: bool,
    #[prop_or_default]
    pub right_item_label: AttrValue,
    #[prop_or_default]
    pub right_item_on_click: Callback<()>,
    #[prop_or_default]
    pub profile_picture: Option<AttrValue>,
}

#[styled_component(CosmoTopBar)]
pub fn top_bar(props: &CosmoTopBarProps) -> Html {
    let top_bar_style = use_style!(
        r#"
display: grid;
grid-template-columns:
    [left-column] var(--page-side-spacing) [content] 1fr [profilepicture] var(
        --profile-picture-size
    )
    [right-column] var(--page-side-spacing);
grid-row: top-menu;
background: linear-gradient(
    to top,
    var(--gradient-bottom-color) 0%,
    var(--gradient-top-color) 100%
);
    "#
    );

    let top_bar_item_style = use_top_bar_item_style();
    let top_bar_item_right_style = use_style!(
        r#"
margin-left: var(--top-menu-item-margin);
border-right-width: 0;
    "#
    );
    let profile_picture_style = use_style!(
        r#"
display: block;
width: var(--profile-picture-size);
height: var(--profile-picture-size);
background: var(--primary-color);
grid-column: profilepicture;
object-fit: cover;
border-bottom-left-radius: var(--border-radius);
border-bottom-right-radius: var(--border-radius);
    "#
    );
    let top_bar_menu_row = use_style!(
        r#"
display: flex;
justify-content: flex-end;
flex-flow: row nowrap;
grid-column: content;
    "#
    );

    let on_click = props.right_item_on_click.clone();
    html!(
        <div class={top_bar_style}>
            <div class={top_bar_menu_row}>
                {for props.children.iter()}
            </div>
            <div class={profile_picture_style.clone()}>
                if let Some(profile_picture) = props.profile_picture.clone() {
                    <img class={profile_picture_style} src={profile_picture} />
                }
            </div>
            if props.has_right_item {
                <a class={classes!(top_bar_item_style, top_bar_item_right_style)} onclick={move |_| on_click.emit(())}>{props.right_item_label.clone()}</a>
            }
        </div>
    )
}

#[hook]
fn use_top_bar_item_style() -> Style {
    use_style!(
        r#"
text-decoration: none;
font-weight: var(--font-weight-menu);
font-family: var(--font-family-menu);
display: flex;
height: 100%;
align-items: center;
padding: 0;
text-transform: uppercase;
color: var(--black);
cursor: pointer;
margin-right: var(--top-menu-item-margin);
font-size: var(--font-size-top-menu);

&:after {
	content: '';
	height: var(--top-menu-item-divider-height);
	width: var(--top-menu-item-divider-width);
	background: var(--black);
	margin-top: var(--top-menu-item-divider-margin-top);
	margin-left: var(--top-menu-item-divider-margin-left);
}

&:last-child:after {
	content: unset;
}
    "#
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTopBarItemProps {
    pub label: AttrValue,
    #[prop_or_default]
    pub on_click: Callback<()>,
}

#[styled_component(CosmoTopBarItem)]
pub fn top_bar_item(props: &CosmoTopBarItemProps) -> Html {
    let style = use_top_bar_item_style();

    let on_click = use_callback(props.on_click.clone(), |_: MouseEvent, on_click| {
        on_click.emit(())
    });

    html!(
        <a class={style} onclick={on_click}>{props.label.clone()}</a>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTopBarItemExternalProps {
    pub label: AttrValue,
    pub href: AttrValue,
}

#[styled_component(CosmoTopBarItemExternal)]
pub fn top_bar_item_external(props: &CosmoTopBarItemExternalProps) -> Html {
    let style = use_top_bar_item_style();

    html!(
        <a class={style} href={props.href.clone()} target="_blank">{props.label.clone()}</a>
    )
}

#[cfg(feature = "with-yew-router")]
#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTopBarItemLinkProps<Route>
where
    Route: Routable + 'static,
{
    pub label: AttrValue,
    pub to: Route,
}

#[cfg(feature = "with-yew-router")]
#[styled_component(CosmoTopBarItemLink)]
pub fn top_bar_item<Route>(props: &CosmoTopBarItemLinkProps<Route>) -> Html
where
    Route: Routable + 'static,
{
    let style = use_top_bar_item_style();

    html!(
        <Link<Route> to={props.to.clone()} classes={style}>{props.label.clone()}</Link<Route>>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoBottomBarItemProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CosmoBottomBarLeftItem)]
pub fn bottom_bar_left_item(props: &CosmoBottomBarItemProps) -> Html {
    html!({for props.children.iter()})
}

#[function_component(CosmoBottomBarRightItem)]
pub fn bottom_bar_right_item(props: &CosmoBottomBarItemProps) -> Html {
    html!({for props.children.iter()})
}

#[derive(Clone, derive_more::From, PartialEq)]
pub enum CosmoBottomBarChildren {
    CosmoBottomBarLeftItem(VChild<CosmoBottomBarLeftItem>),
    CosmoBottomBarRightItem(VChild<CosmoBottomBarRightItem>),
}

#[allow(clippy::from_over_into)]
impl Into<Html> for CosmoBottomBarChildren {
    fn into(self) -> Html {
        match self {
            CosmoBottomBarChildren::CosmoBottomBarLeftItem(child) => child.into(),
            CosmoBottomBarChildren::CosmoBottomBarRightItem(child) => child.into(),
        }
    }
}

impl CosmoBottomBarChildren {
    pub(crate) fn is_left(&self) -> bool {
        matches!(self, CosmoBottomBarChildren::CosmoBottomBarLeftItem(_))
    }

    pub(crate) fn is_right(&self) -> bool {
        matches!(self, CosmoBottomBarChildren::CosmoBottomBarRightItem(_))
    }
}

#[derive(PartialEq, Clone, Default)]
pub enum CosmoBottomBarProgressState {
    #[default]
    Hidden,
    Visible,
    Indeterminate,
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoBottomBarProps {
    #[prop_or_default]
    pub children: ChildrenRenderer<CosmoBottomBarChildren>,
    #[prop_or_default]
    pub progress_top_label: AttrValue,
    #[prop_or_default]
    pub progress_bottom_label: AttrValue,
    #[prop_or_default]
    pub progress_state: CosmoBottomBarProgressState,
    #[prop_or_default]
    pub progress_value: usize,
    #[prop_or_default]
    pub progress_max: usize,
}

#[styled_component(CosmoBottomBar)]
pub fn bottom_bar(props: &CosmoBottomBarProps) -> Html {
    let bottom_bar_style = use_style!(
        r#"
grid-row: bottom-bar;
align-items: center;
display: grid;
grid-template-columns: [left] 1fr [center] 1fr [right] 1fr;
gap: 1rem;
padding-left: var(--page-side-spacing);
padding-right: var(--page-side-spacing);
"#
    );

    let bottom_bar_item_left = use_style!(
        r#"
grid-column: left;
justify-self: left;
display: flex;
gap: 1rem;
align-items: center;
flex-direction: row;
    "#
    );
    let bottom_bar_item_center = use_style!(
        r#"
grid-column: center;
justify-self: center;
display: grid;
justify-items: center;
    "#
    );
    let bottom_bar_item_right = use_style!(
        r#"
grid-column: right;
justify-self: right;
display: flex;
gap: 1rem;
align-items: center;
    "#
    );
    let progress_bar_label = use_style!(
        r#"
font-size: var(--font-size);
color: var(--black);
display: block;
    "#
    );

    let left = props.children.iter().find(|item| item.is_left());
    let right = props.children.iter().find(|item| item.is_right());

    html!(
        <div class={bottom_bar_style}>
            <div class={bottom_bar_item_left}>
                if let Some(left) = left.clone() {
                    {left}
                }
            </div>
            if matches!(props.progress_state, CosmoBottomBarProgressState::Visible | CosmoBottomBarProgressState::Indeterminate) {
                <div class={bottom_bar_item_center}>
                    <span class={progress_bar_label.clone()}>{props.progress_top_label.clone()}</span>
                    <CosmoProgressBar is_indeterminate={props.progress_state == CosmoBottomBarProgressState::Indeterminate} value={props.progress_value} max={props.progress_max} />
                    <span class={progress_bar_label}>{props.progress_bottom_label.clone()}</span>
                </div>
            }
            <div class={bottom_bar_item_right}>
                if let Some(right) = right.clone() {
                    {right}
                }
            </div>
        </div>
    )
}
