.\wasm-bindgen-macroquad.sh pong_clone --release

del docs
ren web docs
cd docs
basic-http-server .