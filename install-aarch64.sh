#!/bin/bash

################################################
# License: GNU General Public License v3.0
# Author: Logan Cammish
################################################

echo Please ensure you have a bin directory before running
cd ~/bin
wget -O ~/bin/reader "https://github.com/logancammish/cli-file-reader-remake/releases/download/0.0.2/reader-aarch64"
chmod -x reader
export PATH=$PATH:~/bin
