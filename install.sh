#!/bin/bash

################################################
# ! Untested currently ! 
# you may have to make modifications
# ! Untested currently !
################################################
# License: GNU General Public License v3.0
# Author: Logan Cammish
################################################

echo This script is untested currently
cd ~/bin
wget -O ~/bin/reader "https://github.com/logancammish/cli-file-reader-remake/releases/download/0.0.1-hf2/reader"
chmod -x reader
export PATH=$PATH:~/bin
