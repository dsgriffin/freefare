# freefare

[![Crates.io](https://img.shields.io/crates/v/freefare.svg?maxAge=2592000)](https://crates.io/crates/freefare)
[![Docs.rs](https://docs.rs/freefare/badge.svg)](https://docs.rs/freefare)

High-level Rust bindings for the [`libfreefare`](https://github.com/nfc-tools/libfreefare) library.

This crate builds on top of [freefare-sys](https://github.com/dsgriffin/freefare-sys) and [nfc](https://github.com/dsgriffin/nfc), with safe owned tag and key wrappers for the common libfreefare workflows.

The high-level API uses `freefare::Result<T>` and `freefare::Error` for wrapper
validation failures and libfreefare-reported errors.

## Installation

Install both native libraries first:

- macOS: `brew install libfreefare libnfc`
- Debian/Ubuntu: `sudo apt install libfreefare-dev libnfc-dev`

Then add:

```toml
[dependencies]
freefare = "1.1.0"
```

## Linking

This crate relies on `freefare-sys` to link `libfreefare`, and on `nfc-sys` to link `libnfc`.

If `libfreefare` is installed in a non-standard location, set `LIBFREEFARE_LIB_DIR`. The legacy `LIBFREEFARE_PATH` variable is also supported through `freefare-sys`.

If `libnfc` is installed in a non-standard location, set `LIBNFC_LIB_DIR`.

## Example

```rust,no_run
use freefare::Freefare;
use nfc::Context;

fn main() -> freefare::Result<()> {
    let context = Context::new()?;
    let device = context.open(None)?;

    for tag in Freefare::get_tags(&device)? {
        println!("tag friendly name: {}", tag.friendly_name()?);
        println!("tag uid: {}", tag.uid()?);
    }

    Ok(())
}
```

```rust,no_run
use freefare::Freefare;
use nfc::Context;

fn main() -> freefare::Result<()> {
    let context = Context::new()?;
    let device = context.open(None)?;
    let mut tags = Freefare::get_tags(&device)?;
    let tag = tags.pop().expect("expected at least one tag");

    tag.classic_connect()?;

    let block = 4;
    let data = [
        0xDE, 0xAD, 0xBE, 0xEF,
        0x00, 0x01, 0x02, 0x03,
        0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0A, 0x0B,
    ];
    tag.classic_write(block, &data)?;

    let mut read_buffer = [0u8; 16];
    tag.classic_read(block, &mut read_buffer)?;
    println!("block {block}: {read_buffer:?}");

    tag.classic_disconnect()?;

    Ok(())
}
```

## API Shape

`freefare` 1.1.0 brings the crate metadata, documentation, and dependency versions in line with the newer `nfc`, `nfc-sys`, and `freefare-sys` releases.

The crate exposes two layers:

- a safe ownership-based API centered on `Tag`, `DesfireKey`, `Mad`, and `TLV`
- legacy compatibility modules for older raw-ish wrapper code
- the raw FFI surface through `freefare::ffi`

As part of the `1.0.0` alignment with `freefare-sys`, wrappers for old non-public `libfreefare` symbols were dropped. The crate now tracks the public header surface exposed by `freefare-sys`.
The `mifare` compatibility module is intentionally de-emphasized in favor of `Tag` and `DesfireKey`.
Some awkward DESFire application-management helpers, especially the opaque AID-pointer flows, are intentionally left lower-level or raw-only because libfreefare does not expose a clean ownership contract for them.

## Safety

The primary top-level API is designed to be safe in the same spirit as `nfc`:

- discovered tags are owned by `Tag` and cannot outlive the `nfc::Device` they came from
- DESFire keys are owned by `DesfireKey` and are freed automatically on drop
- strings and arrays returned by libfreefare are copied into owned Rust values before native memory is released

Advanced or awkward libfreefare entry points that still require raw pointer interop remain available through `freefare::ffi` and the lower-level wrapper modules.

## License

MIT
