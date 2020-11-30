build: 
	js-bindgen --language c bindings/web_console.yaml > headers/web_console.h
	js-bindgen --language c bindings/web_canvas.yaml > headers/web_canvas.h
	js-bindgen --language c bindings/web_dom.yaml > headers/web_dom.h