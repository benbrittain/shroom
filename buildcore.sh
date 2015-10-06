#!/bin/sh
mkdir ".libcore"

rustc -C opt-level=2 -Z no-landing-pads --target x86_64-apple-darwin -g rust/src/libcore/lib.rs --out-dir "./.libcore"
