#!/bin/bash

# Setup
BUILD_PROFILE=${1:-release} # dev, other Cargo.toml profile or release (default)
BUILD_DIR=platform-build
mkdir $BUILD_DIR
cd $BUILD_DIR

# Create the jniLibs build directory
JNI_DIR=jniLibs
mkdir -p $JNI_DIR

# Set up cargo-ndk
cargo install cargo-ndk
rustup target add \
        aarch64-linux-android \
        armv7-linux-androideabi \
        x86_64-linux-android \
        i686-linux-android \
        wasm32-wasi

# By default use wasmtime
bash "../scripts/config_api.sh" wasmtime

# Build the android libraries in the jniLibs directory
for TARGET in armeabi-v7a arm64-v8a x86 x86_64; do
        cargo ndk -o $JNI_DIR --manifest-path ../Cargo.toml \
                -t $TARGET build --profile $BUILD_PROFILE
        # if the exit code is not 0, build with wasmi instead of wasmtime
        if [[ $? != 0 ]]; then
                bash "../scripts/config_api.sh" wasmi
                cargo ndk -o $JNI_DIR --manifest-path ../Cargo.toml \
                        -t $TARGET build --profile $BUILD_PROFILE
                bash "../scripts/config_api.sh" wasmtime # revert wasmtime default
        fi
done

if [[ $BUILD_RUST_WASM_EXAMPLES != false ]]; then
    bash ../scripts/build-rust-wasm-examples.sh $BUILD_PROFILE
fi

# Archive the dynamic libs
cd $JNI_DIR
tar -czvf ../android.tar.gz *
cd -

# Cleanup
rm -rf $JNI_DIR
