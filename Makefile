build: 
	cd crates/js-bindgen/ && cargo run -- --language c ../../bindings/web_console.yaml > ../../headers/web_console.h
	cd crates/js-bindgen/ && cargo run -- --language c ../../bindings/web_canvas.yaml > ../../headers/web_canvas.h
	cd crates/js-bindgen/ && cargo run -- --language c ../../bindings/web_dom.yaml > ../../headers/web_dom.h
	cd crates/js-bindgen/ && cargo run -- --language rust ../../bindings/web_console.yaml > ../../crates/web_console/src/lib.rs
	cd crates/js-bindgen/ && cargo run -- --language assemblyscript ../../bindings/web_console.yaml > ../../assemblyscript/web_console.ts