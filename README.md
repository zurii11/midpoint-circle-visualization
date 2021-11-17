# Midpoint Circle Drawing Algorithm

This was inspired and done by following [tsoding's](https://github.com/tsoding) video, which you can find [here](https://www.youtube.com/watch?v=AJIyRE2vZ_0).

You can read more about midpoint circle drawing algorithm here:
- https://en.wikipedia.org/wiki/Midpoint_circle_algorithm

Images are drawn in PPM format, read about it here:
- https://en.wikipedia.org/wiki/Netpbm
- https://www.cs.swarthmore.edu/~soni/cs35/f13/Labs/extras/01/ppm_info.html

## Quickstart

Cargo works, but because program uses PPM format, no dependencies are need, so you an just use rust compiler.

```console
$ rustc main.rs
$ ./main
```

Program can generate 4 types of pattersn: Checker pattern, stripes, solid circle and hollow circle.

```console
$ ./main -c  // Checker pattern
$ ./main -s  // Stripes pattern
$ ./main -sc // Solid circle
$ ./main -hc // Hollow circe
```

Calling `./main` without any arguments generates all 4 patterns.

`.ppm` images can be viewed by a lot of image viewers, I personally use minimalistic and simple [feh](https://feh.finalrewind.org/).
