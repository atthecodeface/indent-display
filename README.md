# indent-rs

A type and associated traits to enable display of hierarcical
structures with appropriate indentation.

## Examples

```
use indent_display::{Indenter, NullOptions, DefaultIndentedDisplay};
let mut ind = Indenter::new(&std::io::stdout(), "  ", &NullOptions {});
"banana\n".indent(&mut ind);
panic("argh");
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
indent-display = "0.1.0"
```

## Releases

Release notes are available in [RELEASES.md](RELEASES.md).

## License

Licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
