# rsn-fmt

[![CI Status](https://github.com/ModProg/rsn-fmt/actions/workflows/test.yaml/badge.svg)](https://github.com/ModProg/rsn-fmt/actions/workflows/test.yaml)
[![Crates.io](https://img.shields.io/crates/v/rsn-fmt)](https://crates.io/crates/rsn-fmt)
[![Docs.rs](https://img.shields.io/crates/v/template?color=informational&label=docs.rs)](https://docs.rs/rsn-fmt)
[![Documentation for `main`](https://img.shields.io/badge/docs-main-informational)](https://modprog.github.io/rsn-fmt/rsn_fmt/)

## Configuration

Configuration can be done through a `rsnfmt.rsn` or `.rsnfmt.rsn` config file in any parent directory
or in the user's configuration:

| Linux (and similar)                   | Windows                                    | macOS                            |
|---------------------------------------|--------------------------------------------|----------------------------------|
| `$XDG_CONFIG_HOME` or `$HOME/.config` | `$HOME/Library/Application Support`        | `{FOLDERID_RoamingAppData}`      |
| `/home/alice/.config`                 | `/Users/Alice/Library/Application Support` | `C:\Users\Alice\AppData\Roaming` |

The default values are:

```rust
{
    max_width: 60,
    // Normalize all comments to a specific format
    // Possible values: Block, Line, No
    normalize_comments: No,
    // Wrap comments: boolean
    wrap_comments: false,
    // Should formatting preserve empty lines
    // Possible values: One, All, None
    preserve_empty_lines: One,
    // Inherit parent/global configuration: boolean
    inherit: true
}
```
