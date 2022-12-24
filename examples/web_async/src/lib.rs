use web::*;

#[web::main]
async fn main() {
    loop {
        console_log("⏰ tic");
        sleep(1000.0).await;
        console_log("⏰ tock");
        sleep(1000.0).await;
    }
}
