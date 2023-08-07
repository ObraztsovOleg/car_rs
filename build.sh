#!/bin/bash

cross build --target arm-unknown-linux-gnueabihf
rm -rf ./server
mv target/arm-unknown-linux-gnueabihf/debug/server ./