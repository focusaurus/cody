# Cody: Command Line Transcoder

This is a small utility for converting between common data encoding format.

**Usage**: `cody input-format output-format`.

Input data is read from standard input. Transcoded output is written to standard output.

## Examples

```sh
echo "Hello, Cody!" | base64 | cody base64 binary     
Hello, Cody!
```

## Supported Encodings

* binary
* hexadecimal
* base64
* decimal
