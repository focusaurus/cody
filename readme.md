# Cody: Command Line Transcoder

This is a small utility for converting between common data encoding format.

**Usage**: `cody input-format output-format`.

Input data is read from standard input. Transcoded output is written to standard output.

## Examples

```sh
printf fe | cody hexadecimal decimal
254
printf 254 |cody decimal hexadecimal
fe
```

```sh
printf "Hello, Cody!" | cody binary base64  
SGVsbG8sIENvZHkh
printf SGVsbG8sIENvZHkh | cody base64 binary
Hello, Cody!
```

```sh
printf 07ff | cody hexadecimal base64
B/8=
```

## Supported Conversions

* binary to hexadecimal
* binary to base64
* binary to decimal
* hexadecimal to binary
* hexadecimal to base64
* hexadecimal to decimal
* base64 to binary
* base64 to hexadecimal
* decimal to hexadecimal
