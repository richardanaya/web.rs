struct WebGPU;

impl WebGPU {
    pub fn is_available() -> bool {
        js!("
            return (typeof navigator !== "undefined" && navigator.gpu)?1:0;
        ").invoke()
    }
}