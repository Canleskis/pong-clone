cargo build --release --target wasm32-unknown-unknown

set mypath=%cd%
cd C:\Dev\rust\pong-clone\target\wasm32-unknown-unknown\release
copy pong-clone.wasm %mypath%\docs

cd %mypath%/docs

echo %mypath%

basic-http-server .