#!/bin/sh

if [ "$OS" == "$OS_OPENBSD" ]; then
  doas make debug
else
  sudo make debug
fi

./target/debug/ytui hello world 123
