build:
	clang -O3 --target=wasm32 -nostdlib -I./ -Wl,--no-entry -Wl,-allow-undefined -Wl,--export-dynamic -o main.wasm src/main.c
serve:
	python3 -m http.server 8080
lint:
	clang-lint -i src/main.c
deps:
	sudo dnf install clang llvm lld
	