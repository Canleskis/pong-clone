cargo build --release --target wasm32-unknown-unknown

set mypath=%cd%
cd target\wasm32-unknown-unknown\release
copy pong_clone.wasm %mypath%\docs

cd %mypath%/docs

echo %mypath%

basic-http-server .