build: 
	CC=emcc cargo build --target wasm32-unknown-emscripten && cp ./target/wasm32-unknown-emscripten/debug/libretro-common-rs.js ./static/ && cp ./target/wasm32-unknown-emscripten/debug/libretro_common_rs.wasm ./static/

run:
	basic-http-server ./static/

emsdk:
	source /tmp/emsdk/emsdk_env.sh

