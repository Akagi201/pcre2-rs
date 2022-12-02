# pcre2-rs

Now, only tested on linux, can be easily ported to macOS and windowns.

## Build Steps

1. build pcre2 library

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
