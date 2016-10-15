# freefare

[![Crates.io](https://img.shields.io/crates/v/freefare.svg?maxAge=2592000)](https://crates.io/crates/freefare)

Rust bindings for the [libfreefare](https://github.com/nfc-tools/libfreefare) library.

For raw FFI bindings for `libfreefare`, see [freefare-sys](https://github.com/dsgriffin/freefare-sys).

## Installation

Make sure you've got `libnfc` installed ([on Debian/Ubuntu](http://nfc-tools.org/index.php?title=Libnfc#Debian_.2F_Ubuntu), or `brew install libnfc` using Homebrew on macOS, or on [other systems](http://nfc-tools.org/index.php?title=Libnfc#Installation)).

Then install `libfreefare` ([on Debian/Ubuntu](https://github.com/nfc-tools/libfreefare#installation), or `brew install libfreefare` using Homebrew on macOS)

### Cargo.toml

    [dependencies]
    libc = "0.2.0"
    freefare = "0.1.2"
    
## Example Usage

  -
    
## TODO

* Replace any raw pointers
* Provide examples
* Documentation
* Test
  
## Contributing
    
I'm brand new to Rust so any help or constructive information would be really appreciated. Thanks in advance!    
    
## License
    
MIT    