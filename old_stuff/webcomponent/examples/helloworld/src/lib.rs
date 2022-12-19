use web::*;

struct HelloWorld(HTMLElement);

impl CustomElement for HelloWorld {
    fn new(element: HTMLElement) -> Self {
        HelloWorld(element)
    }
    fn connected(&mut self) {
        set_inner_html(self.0, "Hello World!");
    }
}

#[no_mangle]
fn main() {
    HelloWorld::register("hello-world");
}
