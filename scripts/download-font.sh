#!/usr/bin/env bash

VERSION="7.0.4"

archive="./target/newcm-$VERSION.txz"
url="https://download.gnu.org.ua/release/newcm/newcm-$VERSION.txz"

curl --output "$archive" "$url"

tar -C target --use-compress-program xz -xvf "$archive"
