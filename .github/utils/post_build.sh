#!/bin/sh

# this is a workaround to release rust binaries with goreleaser
# ref: https://jondot.medium.com/shipping-rust-binaries-with-goreleaser-d5aa42a46be0s

x86_64_DARWIN=target/x86_64-apple-darwin/release/cl
x86_64_LINUX=target/x86_64-unknown-linux-gnu/release/cl
ARM64_DARWIN=target/aarch64-apple-darwin/release/cl
ARM64_LINUX=target/aarch64-unknown-linux-gnu/release/cl

if [ -f "$x86_64_DARWIN" ]; then
    echo "replacing x86_64 apple bin" >> result.done
    
    error=$(rm -f dist/cl_darwin_amd64_v1/cl \
    && mv $x86_64_DARWIN dist/cl_darwin_amd64_v1/cl 2>&1 )
fi
if [ -f "$x86_64_LINUX" ]; then
    echo "replacing x86_64 linux bin" >> result.done
    
    error=$(rm -f dist/cl_linux_amd64_v1/cl \
    && mv $x86_64_LINUX dist/cl_linux_amd64_v1/cl 2>&1 )
fi
if [ -f "$ARM64_DARWIN" ]; then
    echo "replacing arm64 apple bin" >> result.done
    
    error=$(rm -f dist/cl_darwin_arm64/cl \
    && mv $ARM64_DARWIN dist/cl_darwin_arm64/cl 2>&1 )
fi
if [ -f "$ARM64_LINUX" ]; then
    echo "replacing arm64 linux bin" >> result.done
    
    error=$(rm -f dist/cl_linux_arm64/cl \
    && mv $ARM64_LINUX dist/cl_linux_arm64/cl 2>&1 )
fi

touch result.done
if [ -z "$error" ]; then
    echo "done!" >> result.done
else
    echo $error >> result.done
fi
