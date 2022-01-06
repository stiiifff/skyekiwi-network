// Copyright 2021 @skyekiwi authors & contributors
// SPDX-License-Identifier: GPL-3.0-or-later

const execSync = require('./execSync.cjs');
console.log('$ yarn contract:compile', process.argv.slice(2).join(' '));

function contractBuild() {
  execSync('cd mock-enclave/contract && rustup target add wasm32-unknown-unknown --toolchain nightly');
  execSync('cd mock-enclave/contract && SKIP_WASM_BUILD=1 cargo build --target wasm32-unknown-unknown --release');
  execSync('cp mock-enclave/contract/target/wasm32-unknown-unknown/release/greeting.wasm mock-enclave/wasm/');
}

contractBuild();
