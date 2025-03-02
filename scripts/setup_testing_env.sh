#! /bin/bash

# Download the Pocket IC binary
curl -L https://github.com/dfinity/pocketic/releases/download/7.0.0/pocket-ic-x86_64-linux.gz -o pocket-ic.gz

# Unzip the binary
gzip -d pocket-ic.gz

# Make the binary executable
chmod +x pocket-ic

export POCKET_IC_BIN=$(pwd)/pocket-ic
