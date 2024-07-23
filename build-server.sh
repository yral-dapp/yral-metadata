#!/bin/bash

cd server && cargo build --release --target x86_64-unknown-linux-musl
