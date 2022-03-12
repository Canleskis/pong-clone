.\wasm-bindgen-macroquad.sh pong_clone --release

rmdir docs /q /s
ren web docs
cd docs
basic-http-server .