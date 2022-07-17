use web_console::*;
use web_local_storage::*;

#[no_mangle]
pub fn main() {
    local_storage_clear();
    let a = local_storage_get_item("foo");
    match a {
        Some(s) => console_log(&s),
        None => console_log("nothing in foo"),
    }
    local_storage_set_item("foo", "abc");
    let a = local_storage_get_item("foo");
    match a {
        Some(s) => console_log(&s),
        None => console_log("nothing in foo"),
    }
    local_storage_remove_item("foo");
    let a = local_storage_get_item("foo");
    match a {
        Some(s) => console_log(&s),
        None => console_log("nothing in foo"),
    }
    local_storage_set_item("foo", "abc");
    local_storage_clear();
    let a = local_storage_get_item("foo");
    match a {
        Some(s) => console_log(&s),
        None => console_log("nothing in foo"),
    }
}
