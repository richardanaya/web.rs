use webcomponent::*;

struct HelloWorld(HTMLElement);

impl CustomElement for HelloWorld {
    fn new(element: HTMLElement) -> Self {
        HelloWorld(element)
    }
    fn connected(&mut self) {
        let shadow_dom = attach_shadow(&self.0, true);
        set_html(&shadow_dom, r#"<div>Hello <slot name="fname"></slot>!</div>"#);
        set_html(&self.0, r#"<span slot="fname">Richard</span>"#);
    }
}

#[no_mangle]
fn main() {
    HelloWorld::register("hello-world");
}
