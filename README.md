# cli-file-reader-remake
[![build](https://github.com/logancammish/cli-file-reader-remake/actions/workflows/rust.yml/badge.svg)](https://github.com/logancammish/cli-file-reader-remake/actions/workflows/rust.yml)

Easily read files over CLI, based on my [older Python project](https://github.com/logancammish/cli-file-reader), now written in Rust.



# Installation: 
### <ins>Linux</ins>
1. Head over to [here](https://github.com/logancammish/cli-file-reader-remake/releases/latest) and download your relevant installation script
2. Ensure you have a bin directory in `~/bin`
3. Open a terminal and run `chmod -x [scriptname] && ./[scriptname]`
   
*alternatively (to not have the binary added to your environment PATH):*
1. Head over to [here](https://github.com/logancammish/cli-file-reader-remake/releases/latest) and download your relevant binary
2. Open a terminal and run `chmod -x [binary]`
### <ins>Windows</ins>
1. Download the executable from [here](https://github.com/logancammish/cli-file-reader-remake/releases/latest)

*optional (to add the executable to your environment PATH):*

2. Open "edit system evironment variables" in Control Panel
3. Press "Environment variables" and find "Path" under System variables
4. Press "Edit" then "New" and add the location of the executable

### <ins>MacOS, FreeBSD, Android and others:</ins>
1. Clone this github repository
2. Install Rust [here](https://www.rust-lang.org/tools/install)
3. Open the repository location in a terminal, and run `cargo build --release`
4. You will find your executable in `/target/release`

# Copyright
Copyright © Logan Cammish 2024

License: GPL-3.0


