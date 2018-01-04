#!/bin/bash

cd webassembly && cargo web build --target-asmjs-emscripten \
  && cd ../cordova \
  && cp ../webassembly/target/asmjs-unknown-emscripten/debug/ionic4.js www/js/ionic4.js \
  && cordova run $1 
  
