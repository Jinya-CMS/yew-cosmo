use stylist::yew::{styled_component, use_style};
use yew::prelude::*;

#[styled_component(CosmoProgressRing)]
pub fn progress_ring() -> Html {
    let loader_container_style = use_style!(
        r#"
display: flex;
height: 100%;
width: 100%;
justify-content: center;
align-items: center;
    "#
    );
    let loader_dot_container_style = use_style!(
        r#"
width: 8rem;
height: 8rem;
position: relative;
    "#
    );
    let loader_dot_style = use_style!(
        r#"
width: 8rem;
height: 8rem;
animation: dwl-dot-spin 5s infinite linear both;
animation-delay: calc(var(--i) * 1s / 8 * -1);
rotate: calc(var(--i) * 60deg / 7);
position: absolute;

&::before {
    content: "";
    display: block;
    width: 0.75rem;
    height: 0.75rem;
    background-color: var(--primary-color);
    border-radius: 50%;
    position: absolute;
    transform: translate(-50%, -50%);
    bottom: 0;
    left: 50%;
}

@keyframes dwl-dot-spin {
    0% {
        transform: rotate(0deg);
        animation-timing-function: cubic-bezier(0.390, 0.575, 0.565, 1.000);
        opacity: 1;
    }

    2% {
        transform: rotate(20deg);
        animation-timing-function: linear;
        opacity: 1;
    }

    30% {
        transform: rotate(180deg);
        animation-timing-function: cubic-bezier(0.445, 0.050, 0.550, 0.950);
        opacity: 1;
    }

    41% {
        transform: rotate(380deg);
        animation-timing-function: linear;
        opacity: 1;
    }

    69% {
        transform: rotate(520deg);
        animation-timing-function: cubic-bezier(0.445, 0.050, 0.550, 0.950);
        opacity: 1;
    }

    76% {
        opacity: 1;
    }

    76.1% {
        opacity: 0;
    }

    80% {
        transform: rotate(720deg);
    }

    100% {
        opacity: 0;
    }
}
    "#
    );

    html!(
        <div class={loader_container_style}>
            <div class={loader_dot_container_style}>
                <div style="--i: 0;" class={loader_dot_style.clone()}></div>
                <div style="--i: 1;" class={loader_dot_style.clone()}></div>
                <div style="--i: 2;" class={loader_dot_style.clone()}></div>
                <div style="--i: 3;" class={loader_dot_style.clone()}></div>
                <div style="--i: 4;" class={loader_dot_style.clone()}></div>
                <div style="--i: 5;" class={loader_dot_style.clone()}></div>
            </div>
        </div>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoProgressBarProps {
    #[prop_or(false)]
    pub is_indeterminate: bool,
    #[prop_or(0)]
    pub value: usize,
    #[prop_or(100)]
    pub max: usize,
}

#[styled_component(CosmoProgressBar)]
pub fn progress_bar(props: &CosmoProgressBarProps) -> Html {
    let progress_style = use_style!(
        r#"
--progress-bar-background: repeating-linear-gradient(
    -45deg,
    var(--primary-color),
    var(--primary-color) var(--progress-bar-gradient-width-1),
    var(--primary-color-light) var(--progress-bar-gradient-width-1),
    var(--primary-color-light) var(--progress-bar-gradient-width-2)
);

display: inline-block;
vertical-align: baseline;
appearance: none;
width: var(--progress-bar-width-medium);
height: var(--progress-bar-height);
overflow: hidden;
border: 0;
background-color: var(--control-border-color);
color: var(--primary-color);
border-radius: var(--border-radius);

&::-webkit-progress-bar {
	background: transparent;
}

&[value]::-webkit-progress-value {
	border-radius: var(--border-radius);
	background: var(--progress-bar-background);
}

&::-moz-progress-bar {
	border-radius: var(--border-radius);
	background: var(--progress-bar-background);
}


@media (prefers-reduced-motion: no-preference) {
	&:indeterminate {
		--progress-background: var(--control-border-color)
			linear-gradient(to right, var(--primary-color) 30%, var(--control-border-color) 30%) top
			left/150% 150% no-repeat;
		animation: progressIndeterminate 1s linear infinite;
		background: var(--progress-background);
	}

	&:indeterminate[value]::-webkit-progress-value {
		background: transparent;
	}

	&:indeterminate::-moz-progress-bar {
		background: transparent;
	}
}

@keyframes progressIndeterminate {
	0% {
		background-position: 200% 0;
	}
	100% {
		background-position: -200% 0;
	}
}
    "#
    );

    if !props.is_indeterminate {
        html!(
            <progress class={progress_style} value={props.value.to_string()} max={props.max.to_string()}></progress>
        )
    } else {
        html!(
            <progress class={progress_style}></progress>
        )
    }
}
