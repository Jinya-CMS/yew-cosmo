use yew::prelude::*;
use yew_cosmo::prelude::*;

#[function_component(AboutCosmo)]
pub fn about_cosmo() -> Html {
    html!(
        <>
            <CosmoTitle title="Cosmo CSS by Jinya" />
            <CosmoHeader level={CosmoHeaderLevel::H2} header="What is Cosmo CSS" />
            <CosmoParagraph>
                {"Cosmo CSS is a small CSS library that gives you the ability to style your applications
                following the cosmopolitan design guidelines. These guidelines were developed by
                Microsoft as theme for Silverlight. Later they were adapted by the Zune desktop software
                and were implemented in the .net framework "}
                <CosmoAnchor href="https://github.com/firstfloorsoftware/mui">{"Modern UI for WPF (MUI)"}</CosmoAnchor>
            </CosmoParagraph>
            <CosmoParagraph>
                {"The Cosmopolitan design was always looking good and quite easy to use. With Jinya CMS
                version 18.0.0 the CMS got a new polished administration based on this CSS library."}
            </CosmoParagraph>
            <CosmoParagraph>
                {"If you need a CSS library for your next web application cosmo might be the right fit."}
            </CosmoParagraph>
            <CosmoHeader level={CosmoHeaderLevel::H2} header="What Cosmo CSS is not" />
            <CosmoParagraph>
                {"The cosmopolitan design was developed by Microsoft for desktop and Silverlight
                applications and is not made for websites in the classical sense. Even though we
                enriched Cosmo with typography, there are many things missing you would want for a
                public facing website."}
            </CosmoParagraph>
            <CosmoHeader level={CosmoHeaderLevel::H2} header="How to install" />
            <CosmoParagraph>
                {"You can either just drop the CSS files in your project or install the npm package "}
                <CosmoCode>{"@jinyacms/cosmo"}</CosmoCode>
            </CosmoParagraph>
            <CosmoHeader level={CosmoHeaderLevel::H2} header="Found a bug?" />
            <CosmoParagraph>
                {"If you found a bug feel free to create an issue on Github or on our Taiga instance: "}
                <CosmoAnchor href="https://taiga.imanuel.dev/project/cosmo-css/">{"https://taiga.imanuel.dev/project/cosmo-css/"}</CosmoAnchor>
            </CosmoParagraph>
            <CosmoHeader level={CosmoHeaderLevel::H2} header="License" />
            <CosmoParagraph>{"Like all other Jinya projects, Cosmo CSS is distributed under the MIT License."}</CosmoParagraph>
        </>
    )
}
