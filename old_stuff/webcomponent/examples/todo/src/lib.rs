use js_ffi::*;
use web_console::*;
use web_dom::*;
use webcomponent::*;

struct TodoList {
    items: Vec<TodoItem>,
}

impl Default for TodoList {
    fn default() -> Self {
        TodoList {
            items: vec![TodoItem {
                done: true,
                description: "write some rust".to_string(),
            }],
        }
    }
}

struct TodoItem {
    done: bool,
    description: String,
}

struct TodoListComponent {
    element: HTMLElement,
    shadow_root: HTMLElement,
}

impl CustomElement for TodoListComponent {
    fn new(element: HTMLElement) -> Self {
        TodoListComponent {
            shadow_root: attach_shadow(&element, true),
            element,
        }
    }
    fn connected(&mut self) {
        self.render();
        let state = globals::get::<TodoList>();
        append_html(self.element,r#"<todo-item done="yes"></todo-item>"#);
    }
}

impl TodoListComponent {
    fn render(&mut self) {
        set_html(&self.shadow_root, include_str!("todo-list.html"));
    }
}

struct TodoItemComponent {
    element: HTMLElement,
    shadow_root: HTMLElement,
    is_done: bool,
}

impl CustomElement for TodoItemComponent {
    fn new(element: HTMLElement) -> Self {
        TodoItemComponent {
            shadow_root: attach_shadow(&element, true),
            element: element,
            is_done: false,
        }
    }

    fn connected(&mut self) {
        set_html(&self.shadow_root, include_str!("todo-item.html"));
        let btn = query_selector(&self.shadow_root, "button");
        add_event_listener(
            &btn,
            "click",
            create_callback_0(|| {
                js!(window.alert).invoke_1("I was clicked!");
            }),
        );
        self.render();
    }

    fn observed_attributes() -> Vec<&'static str> {
        vec!["done"]
    }

    fn attribute_changed(
        &mut self,
        name: String,
        _old_value: Option<String>,
        new_value: Option<String>,
    ) {
        if name == "done" {
            if let Some(value) = new_value {
                if value == "yes" {
                    self.is_done = true;
                } else {
                    self.is_done = false;
                }
            } else {
                self.is_done = false;
            }
        }
        self.render();
    }
}

impl TodoItemComponent {
    fn render(&mut self) {
        if self.is_done {
            log("done");
        } else {
            log("not done");
        }
    }
}

#[no_mangle]
fn main() {
    TodoListComponent::register("todo-list");
    TodoItemComponent::register("todo-item");
}
