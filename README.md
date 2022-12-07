# pcre2-rs

[![Rust CI](https://github.com/Akagi201/pcre2-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Akagi201/pcre2-rs/actions/workflows/ci.yml)

Now, only tested on linux, can be easily ported to macOS and windowns.

## Programs

* [pcre2-sys](pcre2-sys) - Rust bindings for PCRE2.
* [processor](processor) - Rust regex processor program.
* [udplog](scripts/udplog.sh) - Shell UDP log receiver, depends on `openbsd-netcat`.

## Usage

First you must be in a linux system.

In terminal one:

```sh
cd scripts
./udplog.sh 8327
```

In terminal two:

```sh
cd processor
cargo run
```

## How to build the pcre2 static clib

build pcre2 library (the repo already includes the built libs and headers for linux)

```sh
git clone https://github.com/PCRE2Project/pcre2.git # or download a release version
cd pcre2
mkdir build
cd build
cmake -DBUILD_STATIC_LIBS=on -DPCRE2_STATIC_PIC=on ..
make
# copy pcre2.h to ./pcre2-sys/include
# copy libpcre2-8.a to ./pcre2-sys/lib
```
