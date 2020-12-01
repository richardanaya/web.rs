build: 
	js-bindgen --language c bindings/web_console.yaml > headers/web_console.h
	js-bindgen --language c bindings/web_canvas.yaml > headers/web_canvas.h
	js-bindgen --language c bindings/web_dom.yaml > headers/web_dom.h
	js-bindgen --language rust bindings/web_console.yaml > crates/web_console/src/lib.rs
	js-bindgen --language assemblyscript bindings/web_console.yaml > assemblyscript/web_console.ts