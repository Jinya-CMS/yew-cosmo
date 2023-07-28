use crate::pages::App;

mod pages;
mod routing;
mod ui;

fn main() {
    yew::Renderer::<App>::new().render();
}
