#!/bin/bash

cross build --target arm-unknown-linux-gnueabihf
scp target/arm-unknown-linux-gnueabihf/debug/server pi@192.169.31.5:~/rust_server/