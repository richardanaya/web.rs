use js::*;

fn console_log(s: &str) {
    let console_log = js!(r#"
        function(s){
            console.log(s);
        }"#);
    console_log.invoke(&[s.into()]);
}

fn random() -> f64 {
    let random = js!(r#"
        function(){
            return Math.random();
        }"#);
    random.invoke(&[])
}

#[no_mangle]
pub fn main() {
    let start_loop = js!(r#"
        function(){
            window.setInterval(()=>{
                this.module.instance.exports.run_loop();
            }, 1000)
        }"#);
    start_loop.invoke(&[]);
}

#[no_mangle]
pub fn run_loop(){
    console_log(&format!("⏰ {}", random()));
}
