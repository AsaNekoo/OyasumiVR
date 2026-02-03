#!/bin/bash
set -e
wget -qO- "https://cef-builds.spotifycdn.com/cef_binary_142.0.14%2Bgceaf578%2Bchromium-142.0.7444.163_linux64_minimal.tar.bz2"| tar xvjf -
mv cef_binary_142.0.14+gceaf578+chromium-142.0.7444.163_linux64_minimal/ cef
mv cef/Resources/* cef/
mv cef/Release/* cef/
strip cef/*.so
mv cef/ resources/sidecars/cef
