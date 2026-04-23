# freefare

[![Crates.io](https://img.shields.io/crates/v/freefare.svg?maxAge=2592000)](https://crates.io/crates/freefare)
[![Docs.rs](https://docs.rs/freefare/badge.svg)](https://docs.rs/freefare)

High-level Rust bindings for the [`libfreefare`](https://github.com/nfc-tools/libfreefare) library.

This crate builds on top of [freefare-sys](https://github.com/dsgriffin/freefare-sys) and keeps the common libfreefare workflows a little more ergonomic with `Result`-returning wrappers and Rust slices where the C API allows it.

It is still a relatively thin wrapper. Many operations continue to use raw `FreefareTag` handles and inherit the lifetime and ownership rules of the native library.

The high-level API uses `freefare::Result<T>` and `freefare::Error` for wrapper
validation failures and libfreefare-reported errors.

## Installation

Install both native libraries first:

- macOS: `brew install libfreefare libnfc`
- Debian/Ubuntu: `sudo apt install libfreefare-dev libnfc-dev`

Then add:

```toml
[dependencies]
freefare = "1.0.0"
```

## Linking

This crate relies on `freefare-sys` to link `libfreefare`, and on `nfc-sys` to link `libnfc`.

If `libfreefare` is installed in a non-standard location, set `LIBFREEFARE_LIB_DIR`. The legacy `LIBFREEFARE_PATH` variable is also supported through `freefare-sys`.

If `libnfc` is installed in a non-standard location, set `LIBNFC_LIB_DIR`.

## Example

```rust,no_run
use freefare::freefare::Freefare;
use freefare::mifare::Mifare;

fn main() -> freefare::Result<()> {
    let tag = todo!("obtain a valid FreefareTag from libnfc/libfreefare discovery");

    println!("tag friendly name: {}", Freefare::get_tag_friendly_name(tag)?);

    let block = 4;
    let data = [
        0xDE, 0xAD, 0xBE, 0xEF,
        0x00, 0x01, 0x02, 0x03,
        0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0A, 0x0B,
    ];
    Mifare::classic_write(tag, block, &data)?;

    let mut read_buffer = [0u8; 16];
    Mifare::classic_read(tag, block, &mut read_buffer)?;
    println!("block {block}: {read_buffer:?}");

    Mifare::classic_disconnect(tag)?;

    Ok(())
}
```

## API Shape

`freefare` 1.0.0 brings the crate metadata, documentation, and dependency versions in line with the newer `nfc`, `nfc-sys`, and `freefare-sys` releases.

The crate exposes two layers:

- the existing wrapper modules such as `freefare`, `mifare`, `mad`, and `tlv`
- the raw FFI surface through `freefare::ffi`

As part of the `1.0.0` alignment with `freefare-sys`, wrappers for old non-public `libfreefare` symbols were dropped. The crate now tracks the public header surface exposed by `freefare-sys`.
Two `mifare_application_*` sector-list helpers are intentionally left raw-only through `freefare::ffi` because the native API does not document a length-bearing wrapper contract for those returned lists clearly enough for this crate to model them confidently as safe Rust collections.

## Safety

This crate is more ergonomic than `freefare-sys`, but it is not a fully safe ownership wrapper in the same sense as `nfc`.

Callers still need to ensure that:

- raw tag and key handles remain valid for the duration of each call
- native memory returned by libfreefare is only freed through the correct libfreefare function
- device and tag lifetimes are coordinated correctly with the underlying `libnfc` session
- raw-only APIs exposed through `freefare::ffi` may have additional ownership requirements that are not modeled by the high-level wrapper layer

## License

MIT
