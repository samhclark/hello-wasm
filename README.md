# WebAssembly Barcode Generator

It's really bad code, but it works! Needs a lot of cleanup, refactoring. I didn't wanna spend time on that early in
the process. There were too many unknowns for me to know what it would look like. 

Anyway, if you have Rust (and have already `cargo install wasm-pack`), `just`, and `npm` installed, you can build and
run this with: 

```
just run
```

Check the `Justfile` for the commands to run if you don't have Just. It's just a couple. 

# Credits

I followed Mozilla's Rust-WebAssembly tutorial at first
https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

Then for the actual barcode parts it was Wikipedia 
https://en.wikipedia.org/wiki/Code_128

And University of Washington 
https://courses.cs.washington.edu/courses/cse370/01au/minirproject/BarcodeBattlers/barcodes.html