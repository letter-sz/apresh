#!/bin/bash

git clean -Xdf
dfx start --clean --background
bun i
bun initial
bun dev