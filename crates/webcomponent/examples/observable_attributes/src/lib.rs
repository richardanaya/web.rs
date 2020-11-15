use webcomponent::*;

struct HelloPerson(HTMLElement);

impl CustomElement for HelloPerson {
    fn new(element: HTMLElement) -> Self {
        HelloPerson(element)
    }

    fn observed_attributes() -> Vec<&'static str> {
        vec!["first_name"]
    }

    fn connected(&mut self) {
        self.render();
    }

    fn attribute_changed(
        &mut self,
        _name: String,
        _old_value: Option<String>,
        _new_value: Option<String>,
    ) {
        self.render();
    }
}

impl HelloPerson {
    fn render(&mut self) {
        let first_name = get_attribute(&self.0, "first_name").unwrap_or("human".to_string());
        let msg = "Hello ".to_string() + &first_name;
        set_html(&self.0, &msg);
    }
}

#[no_mangle]
fn main() {
    HelloPerson::register("hello-person");
}
