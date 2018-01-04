#[macro_use]
extern crate fraw;

#[macro_use]
extern crate fraw_component_derive;

use fraw::Fraw;
use fraw::component::Component;
use fraw::html::Tag;

#[derive(Default)]
#[derive(FrawComponent)]
#[fraw_selector = "page"]
struct Page {}
impl Component for Page {
    fn render(&self) -> Tag {
        view! { (self) => {
            <ion_app>
                <ion_page class="show-page">
                    <ion_header>
                        <ion_toolbar>
                            <ion_title>{ "Ionic 4" }</ion_title>
                        </ion_toolbar>
                    </ion_header>
                    <ion_content class="content">
                        <ion_list>
                            <ion_item>{ "Test" }</ion_item>
                            <ion_item>{ "Test" }</ion_item>
                            <ion_item>{ "Test" }</ion_item>
                            <ion_item>{ "Test" }</ion_item>
                        </ion_list>
                    </ion_content>
                </ion_page>
            </ion_app>
        } }
    }
}

fn main() {
    Fraw::register_alias("ion_app", "ion-app");
    Fraw::register_alias("ion_page", "ion-page");
    Fraw::register_alias("ion_header", "ion-header");
    Fraw::register_alias("ion_item", "ion-item");
    Fraw::register_alias("ion_list", "ion-list");
    Fraw::register_alias("ion_title", "ion-title");
    Fraw::register_alias("ion_content", "ion-content");
    Fraw::register_alias("ion_toolbar", "ion-toolbar");

    let fraw = Fraw::init("body", Box::new(Page{}));

    fraw.run();
}
