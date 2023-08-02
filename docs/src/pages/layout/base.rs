use yew::prelude::*;

use yew_cosmo::prelude::*;

use crate::ui::CosmoDocsPre;

#[function_component(BaseLayout)]
pub fn base_layout() -> Html {
    html!(
        <>
            <CosmoTitle title="Base layout and menu" />
            <CosmoParagraph>
                {"Cosmo Yew has a very opinionated layout and to fully use its menu structure you need to
                follow it."}
            </CosmoParagraph>
            <CosmoParagraph>
                {r#"The menu consists of two levels of navigation and allows for a "backstage" menu. The top
                level navigation should navigate between your applications areas whereas the second
                level navigation should navigate between your actual pages."#}
            </CosmoParagraph>
            <CosmoParagraph>
                {"The top bar navigation, left from the square, can be used to send you users into
                settings. In Jinya CMS it is used to toggle between the frontstage, where the artist
                manages their content and the backstage, where the Jinya CMS installation can be
                administered."}
            </CosmoParagraph>
            <CosmoParagraph>
                {"To display status information, for example during file upload, Cosmo Yew features a
                bottom bar. Even though the main usage of the bottom bar is to show progress, it can
                also be used to display buttons that should work on all pages the same."}
            </CosmoParagraph>
            <CosmoDocsPre>{r#"<BrowserRouter>
    <BounceRoot>
        <CosmoPageLayout>
            <HelmetBridge default_title="Cosmo Yew" format_title={format_title} />
            <CosmoTopBar has_right_item={true} right_item_label="Logout">
                <CosmoTopBarItemExternal href="https://github.com/Jinya-CMS/cosmo-css" label="Github" />
                <CosmoTopBarItemExternal href="https://gitlab.imanuel.dev/jinya-cms/cosmo-css" label="GitLab" />
                <CosmoTopBarItemExternal href="https://crates.io/crates/yew-cosmo" label="Crate" />
            </CosmoTopBar>
            <CosmoMenuBar>
                <CosmoMainMenu>
                    <CosmoMainMenuItemLink<DocsRoute> to={DocsRoute::CosmoRoot} label="Cosmo" is_active={true} />
                    <CosmoMainMenuItemLink<DocsRoute> to={DocsRoute::ControlsRoot} label="Controls" />
                    <CosmoMainMenuItemLink<DocsRoute> to={DocsRoute::LayoutRoot} label="Layout" />
                </CosmoMainMenu>
                <CosmoSubMenuBar>
                    <CosmoSubMenuItemLink<CosmoRoute> to={CosmoRoute::About} label="About Cosmo" is_active={true} />
                    <CosmoSubMenuItemLink<CosmoRoute> to={CosmoRoute::Typography} label="Typography" />
                    <CosmoSubMenuItemLink<CosmoRoute> to={CosmoRoute::Theme} label="Theme" />
                    <CosmoSubMenuItemLink<CosmoRoute> to={CosmoRoute::Customize} label="Customize" />
                </CosmoSubMenuBar>
            </CosmoMenuBar>
            <CosmoPageBody>
                <CosmoTitle title="Base layout and menu" />
            </CosmoPageBody>
            <CosmoBottomBar progress_state={CosmoBottomBarProgressState::Indeterminate} progress_bottom_label="Bottom label" progress_top_label="Top label">
                <CosmoBottomBarLeftItem>
                    <CosmoButton label="I am a button on the left" />
                    <CosmoStrong>{"Hello World!"}</CosmoStrong>
                </CosmoBottomBarLeftItem>
                <CosmoBottomBarRightItem>
                    <CosmoCircleButton title="I am a right button" icon={IconId::LucideLeaf} />
                </CosmoBottomBarRightItem>
            </CosmoBottomBar>
        </CosmoPageLayout>
    </BounceRoot>
</BrowserRouter>"#}</CosmoDocsPre>
        </>
    )
}