#!/bin/bash

echo Please ensure you have a bin directory before running
cd ~/bin
wget -O ~/bin/reader "https://github.com/logancammish/cli-file-reader-remake/releases/download/0.0.2/reader"
sudo chmod -x reader
export PATH=$PATH:~/bin
