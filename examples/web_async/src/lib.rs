use web::*;

#[web::main]
async fn main() {
    let canvas = query_selector("#canvas");
    let ctx = CanvasContext::from_element(&canvas);

    // we can spawn concurrent
    coroutine(async move {
        loop {
            console_log("tik");
            sleep(1000).await;
            console_log("tok");
            sleep(1000).await;
        }
    });

    loop {
        // Draw a random color rect
        ctx.set_fill_style(&format!(
            "rgb({}, {}, {})",
            random() * 255.0,
            random() * 255.0,
            random() * 255.0
        ));
        ctx.fill_rect(
            random() * 500.0,
            random() * 500.0,
            random() * 500.0,
            random() * 500.0,
        );
        wait_til_animation_frame().await;
    }
}
