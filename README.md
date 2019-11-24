# matches2

[![Crates.io](https://img.shields.io/crates/v/matches2.svg)](https://crates.io/crates/matches2)
[![License](https://img.shields.io/crates/l/matches2.svg)](https://raw.githubusercontent.com/Laaas/matches2/master/LICENSE)

This is a fork of the matches crate with an extra `unwrap_match!` and `option_match!` macro,
and also better error messages for `assert_matches`.

[Documentation](https://docs.rs/matches2)

## `unwrap_match!` macro

The `unwrap_match!` macro is a general unwrap, used as such:
```rust
	let output = unwrap_match!(input, AnEnum::Variant(a) | AnEnum::OtherVariant(a) if a < 5 * 2 => a);
```

If it fails, it emits a descriptive error including the pattern and the input,
for this reason `input` **must** implement `Debug`, unless you provide a custom error message
as the last arguments, in the same way you'd use `format!`.

## `option_match!` macro

The `option_match!` macro is like `unwrap_match!`, except instead of failing,
it just returns an `Option`.
```rust
	let output: Option<_> = option_match!(input, AnEnum::Variant(a) | AnEnum::OtherVariant(a) if a < 5 * 2 => a);
```

## Error message improvements

The original matches crate would emit horrible errors when assertions failed,
outputting a pattern such as `Some(_)` as `Some ( _ )`. This version has properly
formatted errors, so you will never experience this again.
