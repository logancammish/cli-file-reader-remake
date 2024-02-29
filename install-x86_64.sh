#!/bin/bash

if [ -d ~/bin ]; then
  echo "bin directory exists, continuing..."
else 
  echo "bin directory does not exist, creating..."
  mkdir ~/bin
  echo "continuing..."
fi

cd ~/bin
wget -O ~/bin/reader "https://github.com/logancammish/cli-file-reader-remake/releases/download/0.0.2/reader"
sudo chmod -x reader
export PATH=$PATH:~/bin
