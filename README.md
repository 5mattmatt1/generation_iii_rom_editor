# Setting up Development Environment
## SDL2
This project relies on SDL2 for generation of Bitmaps 
for tilesets at the moment.

### Ubuntu
```
apt-get install libsdl2-dev
```

### Windows
https://www.libsdl.org/download-2.0.php

## Rust
https://www.rust-lang.org/tools/install

## Build
```
cargo build
```

# TODO
Choose between the following patterns
## Pattern 1
Load header and then pass in that header to a data constructor
+ Used a lot in MEH, but seems to also be part of a multiple constructor scheme.
## Pattern 2
Have data constructor call header constructor and store it.
+ Makes more sense as we now only have one top level call which could help in future abstraction
+ Currently the unused version.
