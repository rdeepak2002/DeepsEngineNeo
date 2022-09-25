#!/bin/sh
# build
cd ..
(rm -rf ./build-scripts/web-dist) && (EMCC_CFLAGS="-s ASSERTIONS -s LLD_REPORT_UNDEFINED -s ALLOW_MEMORY_GROWTH=1 -s FULL_ES3=1 -s USE_WEBGL2=1 -s MIN_WEBGL_VERSION=2 -s MAX_WEBGL_VERSION=2 -s USE_SDL=2" CARGO_TARGET_DIR=./build-scripts/web-dist cargo build --target wasm32-unknown-emscripten --verbose)
cd ./build-scripts/ || exit 1

# notify
echo "Web build complete!"

# serve
if [ "$1" = "-o" ]
then
  # run server to if -o argument is passed in
  cd ..
  cd ./build-scripts/ || exit 1
  http-server -s --cors='Access-Control-Allow-Origin: *' -o -c-1
else
  # otherwise tell user they can run a server if they want to
  echo "After installing http-server, feel free to run the following command:"
  echo "cd $PWD/build-scripts && http-server -s --cors='Access-Control-Allow-Origin: *' -o -c-1"
fi
