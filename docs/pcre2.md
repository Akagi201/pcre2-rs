# pcre2 related

## UTF-8, UTF-16, or UTF-32

In the original PCRE library, UTF-16 and UTF-32 support were added in later versions through additional functions prefixed with `pcre16_` and `pcre32_`. In PCRE2, all functions are prefixed with `pcre2_` and suffixed with `_8`, `_16`, or `_32` to select 8-bit, 16-bit, or 32-bit code units. If you compile PCRE2 from source, you need to pass `--enable-pcre2-16` and `--enable-pcre2-32` to the configure script to make sure the `_16` and `_32` functions are available.

8-bit, 16-bit, or 32-bit code units means that PCRE2 interprets your string as consisting of single byte characters, double byte characters, or quad byte characters. To work with UTF-8, UTF-16, or UTF-32, you need to use the functions with the corresponding code unit size, and pass the `PCRE2_UTF` option to `pcre2_compile` to allow characters to consists of multiple code units. UTF-8 characters consist of 1 to 4 bytes. UTF-16 characters consist of 1 or 2 words.

If you want to call the PCRE2 functions without any suffix, as they are shown below, then you need to define `PCRE2_CODE_UNIT_WIDTH` as 8, 16, or 32 to make the functions without a suffix use 8-bit, 16-bit, or 32-bit code units. Do so before including the library, like this:

```c
#define PCRE2_CODE_UNIT_WIDTH 8
#include "pcre2.h"
```

The functions without a suffix always use the code unit size you’ve defined. The functions with suffixes remain available. So your application can use regular expressions with all three code unit sizes. But it is important not to mix them up. If the same regex needs to be matched against UTF-8 and UTF-16 strings, then you need to compile it twice using `pcre_compile_8` and `pcre_compile_16` and then use the compiled regexes with the corresponding `pcre_match_8` and `pcre_match_16` functions.

## Refs

* <https://www.regular-expressions.info/pcre2.html>
* <https://github.com/PCRE2Project/pcre2/blob/master/src/pcre2demo.c>
