use web::*;

#[no_mangle]
pub fn main() {
    let body = query_selector("body");
    element_set_inner_html(&body, "Click anywhere to push state");

    let location = location_url();
    element_add_click_listener(&body, move |_| {
        let body2 = query_selector("body");
        history_push_state("1",&(location.clone()+&random().to_string()));
        element_set_inner_html(&body2, "I arrive by pushing state, notice url, click again to go deeper");
    });
    add_history_pop_state_event_listener(move |_| {
        let body2 = query_selector("body");
        element_set_inner_html(&body2, "I arrive by hitting back without reload");
        console_log(&location_url())
    });
}
