use js_ffi::*;
use webcomponent::*;

struct LoudButton {
    element: HTMLElement,
}

impl CustomElement for LoudButton {
    fn new(element: HTMLElement) -> Self {
        LoudButton { element }
    }
    fn connected(&mut self) {
        set_html(&self.element, "<button>Shout!</button>");
        js!(Node.prototype.addEventListener).call_2(
            &self.element,
            "click",
            create_callback_0(|| {
                js!(window.alert).invoke_1("I was clicked!");
            }),
        );
    }
}

#[no_mangle]
fn main() {
    LoudButton::register("loud-button");
}
