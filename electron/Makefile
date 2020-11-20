build: tmp/electron.zip dist/js-wasm.js dist/app.wasm dist/index.html dist/index.js

run: build
	cd dist && ./electron .

 dist/index.js: index.js
	cp index.js  dist/index.js

dist/app.wasm: src/**.rs
	@RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
	@cp target/wasm32-unknown-unknown/release/example.wasm dist/app.wasm

dist/js-wasm.js:
	wget -O dist/js-wasm.js https://unpkg.com/js-wasm/js-wasm.js

tmp/electron.zip:
	mkdir -p tmp
	mkdir -p dist
	wget -O tmp/electron.zip https://github.com/electron/electron/releases/download/v11.0.2/electron-v11.0.2-linux-x64.zip
	cd tmp && unzip electron.zip -d ../dist

clean: 
	rm -rf dist
	rm -rf tmp

dist/index.html: index.html
	cp index.html dist/index.html