# freefare

[![Crates.io](https://img.shields.io/crates/v/freefare.svg?maxAge=2592000)](https://crates.io/crates/freefare)

Rust bindings for the [libfreefare](https://github.com/nfc-tools/libfreefare) library.

For raw FFI bindings for `libfreefare`, see [freefare-sys](https://github.com/dsgriffin/freefare-sys).

## Requirements

You need to install `libfreefare` and `libnfc` before using this crate.

### Ubuntu + APT

```bash
sudo apt install libfreefare-dev libnfc-dev
```

### MacOS + Homebrew

```bash
brew install libfreefare libnfc
```

### Custom Paths

If the two libraries above are not installed in the standard APT or Homebrew locations, you can override the following environment variables:

* **LIBFREEFARE_PATH**: Path to libfreefare (e.g. **/path/to/libfreefare/lib**).
* **LIBNFC_PATH**: Path to libnfc (e.g. **/path/to/libnfc/lib**).

See `build.rs` for more details on how it works if needed.

## Usage

Add both `libc` and `freefare` as dependencies in your `Cargo.toml`. 

```toml
[dependencies]
libc = "0.2.0"
freefare = "0.3.0"
```

## Example

```rust
extern crate freefare;

use freefare::freefare::Freefare;
use freefare::mifare::Mifare;

fn main() {
    let tag = /* Assume a valid FreefareTag obtained elsewhere */

    // Get friendly name of a tag
    match Freefare::get_tag_friendly_name(tag) {
        Ok(name) => println!("Tag friendly name: {}", name),
        Err(e) => eprintln!("Failed to get tag friendly name: {}", e),
    };

    // Write data to a Mifare Classic block
    let block = 4; // Example block number
    let data: [u8; 16] = [0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B];
    match Mifare::classic_write(tag, block, &data) {
        Ok(_) => println!("Data written to block {}.", block),
        Err(e) => eprintln!("Failed to write data: {}", e),
    }

    // Read data from a Mifare Classic block
    let mut read_buffer = [0u8; 16];
    match Mifare::classic_read(tag, block, &mut read_buffer) {
        Ok(_) => println!("Read data from block {}: {:?}", block, read_buffer),
        Err(e) => eprintln!("Failed to read data: {}", e),
    }

    // Disconnect the tag
    match Mifare::classic_disconnect(tag) {
        Ok(_) => println!("Tag disconnected."),
        Err(e) => eprintln!("Failed to disconnect tag: {}", e),
    }

    // Etc...
}

```

## Contributing

If you've found a bug or have an idea, feel free to open an Issue. If you've got a fix or feature ready, open a PR. Thanks!

## License

MIT
