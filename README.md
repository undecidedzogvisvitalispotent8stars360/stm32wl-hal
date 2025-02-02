# stm32wl-hal

Embedded rust HAL (hardware abstraction layer) for the STM32WL.

This is a **work in progress**, it is unstable, incomplete, and (mostly) untested.

**Use at your own risk.**

There should be enough code for very basic:

* LoRa TX
* (G)FSK TX
* (G)FSK RX
* ECDSA signing
* ECDSA verification
* Cryptographically secure random number generation
* AES encryption
* AES decryption

## Why upload incomplete code?

This has uploaded to github in an incomplete state because this is better than
nothing for other people developing on these new chips.

I am also hoping to find other people using these chips with rust who want to
collaborate on creating a HAL :)

## On-Target Tests

Workspaces with `testsuite` in the name denote an on-target test.

On-target tests use the NUCLEO-WL55JC2, you can find a place purchase this
[here](https://www.st.com/en/evaluation-tools/nucleo-wl55jc.html#sample-buy).

### On-Target Quickstart

The on-target tests use [defmt-test].

* `cargo install flip-link` ([flip-link])
* `cargo install probe-run` ([probe-run])
* `rustup target add --toolchain stable thumbv7m-none-eabi` ([rustup])
* (Linux users only) udev rules are available at [newAM/nucleo-wl55jc2-rs]
* Connect the nucleo board to your PC via USB.
* `cargo test -p pka-testsuite --target thumbv7em-none-eabi`

Sample output:
```text
         INFO  (1/2) running `ecdsa_sign`...
└─ test::tests::__defmt_test_entry @ tests/test.rs:48
         INFO  (2/2) running `ecdsa_verify`...
└─ test::tests::__defmt_test_entry @ tests/test.rs:48
         INFO  all tests passed!
└─ test::tests::__defmt_test_entry @ tests/test.rs:48
```

#### SubGhz Tests

These tests require two nucleo boards, one for transmitting, and one for
receiving.

Run `subghz-testsuite-rx` before `subghz-testsuite-tx`.

Assuming both are connected to the same system you will have to pass a specific
probe to each.

```text
$ probe-run --list-probes
The following devices were found:
[0]: STLink V3 (VID: 0483, PID: 374e, Serial: 001D00145553500A20393256, STLink)
[1]: STLink V3 (VID: 0483, PID: 374e, Serial: 002900205553500A20393256, STLink)
$ cargo test -p subghz-testsuite-rx --target thumbv7em-none-eabi -- --probe 001D00145553500A20393256
$ cargo test -p subghz-testsuite-tx --target thumbv7em-none-eabi -- --probe 002900205553500A20393256
```

## Unit Tests

Off-target unit tests use the built-in cargo framework.
The only difference is you must specify the MCU as a feature.

```bash
cargo test --features stm32wl5x_cm4
```

## Examples

All examples run on the NUCLEO-WL55JC2.
Examples are located in the `examples` crate.

```bash
cargo run --example gpio-button -p examples --target thumbv7em-none-eabi
```

[newAM/nucleo-wl55jc2-rs]: https://github.com/newAM/nucleo-wl55jc2-rs
[defmt-test]: https://crates.io/crates/defmt-test
[flip-link]: https://github.com/knurling-rs/flip-link
[probe-run]: https://github.com/knurling-rs/probe-run
[rustup]: https://rustup.rs/
