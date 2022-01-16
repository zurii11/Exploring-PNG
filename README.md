# Exploring PNG format

I'm just exploring PNG format here. This program can read PNG and inject custom chunk there. It almost works, it injects a custom chunk with text but at a wrong place so CRC gets messed up. Will fix that sometime later.

This was made by following [tsoding's](https://www.youtube.com/watch?v=M9ZwuIv3xz8) video on this topic. Awesome guy, check him out.

PNG Specification: http://libpng.org/pub/png/spec/1.2/PNG-Contents.html

## Quickstart

```console
$ rustc main.rs
$ ./main input.png output.png
```
