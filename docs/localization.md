# The Localization File Format: Version #0

All multi-byte numbers are big-endian and unsigned unless otherwise mentioned

### Header Region
```
2 byte: file version
4 byte: image data region offset
4 byte: text data region offset 
4 byte: Version - Major
4 byte: Version - Minor
4 byte: Version - Patch
1 byte: length of anglicized name (bytes)
  N bytes: bytes of anglicized name [UTF-8]
4 byte: length of native name (bytes)
  N bytes: bytes of native name [UTF-8]
4 byte: length of authors (bytes)
  N bytes: bytes of authors [UTF-8]
```
### Image Data Region
```
48 * 24 * 4 bytes: 8-bit R G B A of each pixel, in row-major order
```

### Text Data Region
```
4 byte: Number of text elements
K times: text elem
  4 byte SIGNED: which text enum value this fits
  4 byte: length of translation (bytes)
    N bytes: bytes of translation [UTF-8]
  2 byte: font size of text
  1 byte: modifications [&1=bold, &2=italic, &4=underline, &8=strikethrough]
  1 byte: red color channel
  1 byte: blue color channel
  1 byte: green color channel

```