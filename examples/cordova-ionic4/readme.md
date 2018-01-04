# What is it ?

Just a POC for the fun of a rust code (with my lib `fraw`) into asmjs, working with ionic4 components in cordova. 

# How to build ?

- Install and configure [Fraw](https://github.com/peekmo/fraw) and [wasm for rust](https://hackernoon.com/compiling-rust-to-webassembly-guide-411066a69fde)
- Install [cordova](https://cordova.apache.org/#getstarted)
- Install dependencies `cd cordova && npm install && cd ..`
- Run script (Unix) `./build.sh <platform>` (where `<platform>` is `ios` or `android`)
