#!/bin/sh
## create folder for web build
#rm -rf ./web-dist
#mkdir -p ./web-dist
## build web version of engine
#cd ..
#cargo build --target wasm32-unknown-unknown --verbose
## navigate back to distribution folder
#cd build-scripts || exit 1
## use wasm-bindgen to tweak generated javascript
#wasm-bindgen ../target/wasm32-unknown-unknown/debug/DeepsEngineNeo.wasm --out-dir ./web-dist --target web
## copy index.html to web build folder
#cp ./index.html ./web-dist
## notify user of the web build being complete
#echo "Web build complete!"
#
#if [ "$1" = "-o" ]
#then
#  # run server to if -o argument is passed in
#  cd web-dist || exit 1
#  http-server -s --cors='Access-Control-Allow-Origin: *' -o
#else
#  # otherwise tell user they can run a server if they want to
#  echo "After installing http-server, feel free to run the following command:"
#  echo "cd $PWD/web-dist && http-server -s --cors='Access-Control-Allow-Origin: *' -o"
#f

(rm -rf ./build-scripts/web-dist) && (EMCC_CFLAGS="-s ASSERTIONS -s LLD_REPORT_UNDEFINED -s ALLOW_MEMORY_GROWTH=1 -s FULL_ES3=1 -s USE_WEBGL2=1 -s MIN_WEBGL_VERSION=2 -s MAX_WEBGL_VERSION=2 -s USE_SDL=2" CARGO_TARGET_DIR=./build-scripts/web-dist cargo build --target wasm32-unknown-emscripten --verbose)