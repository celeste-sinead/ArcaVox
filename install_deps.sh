#!/bin/bash
if [ "$1" == "ubuntu-latest" ]; then
    apt install libasound2-dev libfontconfig-dev
elif [ "$1" == "macos-latest" ]; then
    echo "Nothing to do"
else
    echo "Unknown OS: $1"
    exit 1
fi