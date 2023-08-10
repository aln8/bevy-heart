wasm-gen:
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --out-dir ./out/ --target web --typescript target/wasm32-unknown-unknown/release/bevy-heart.wasm

build-docker:
	cp -r deploy/config deploy/index.html assets out/
	docker build -f deploy/Dockerfile -t heart:0.0.1 .

run-docker:
	docker run --rm -it -p 8000:80 heart:0.0.1 
