#!/bin/bash

git clean -Xdf && \
dfx start --clean --background && \
bun wasm && \
bun i && \
bun initial && \
bun dev