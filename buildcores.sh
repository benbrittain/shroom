export RUST_TARGET_PATH=$(pwd)/targets
RUST_TARGET_PATH=$(pwd)/targets

RUSTC_DIR=$(pwd)/rust

make_target_libs() {
    sudo mkdir -p /usr/lib/rustlib/$1/lib
    pushd $RUSTC_DIR >/dev/null

    pushd src/compiler-rt >/dev/null
    [ -d build ] || mkdir build
    make -j3 ProjSrcRoot=$(pwd) ProjObjRoot=$(pwd)/build TargetTriple=$2 triple-builtins
    cp build/triple/builtins/libcompiler_rt.a ~/.multirust/toolchains/nightly/lib/rustlib/$1/lib
    popd >/dev/null

    pushd src/libcore >/dev/null
    rustc -O --target=$1 lib.rs
    cp libcore.rlib ~/.multirust/toolchains/nightly/lib/rustlib/$1/lib
    popd >/dev/null
}

make_target_libs i686-sel4-unknown i686-unknown-linux-gnu
