# fast-glob

## Introduce

A high-performance glob matching crate for Rust based on [`devongovett/glob-match`](https://github.com/devongovett/glob-match). 

**Key Features:**

- Up to 60% performance improvement.
- Supports more complete and well-rounded features.

## Examples
```rust
use fast_glob::glob_match;

let glob = "some/**/n*d[k-m]e?txt";
let path = "some/a/bigger/path/to/the/crazy/needle.txt";

assert!(glob_match(glob, path));
```

## Syntax

| Syntax  | Meaning                                                                                                                                                                                             |
| ------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `?`     | Matches any single character.                                                                                                                                                                       |
| `*`     | Matches zero or more characters, except for path separators (e.g. `/`).                                                                                                                             |
| `**`    | Matches zero or more characters, including path separators. Must match a complete path segment (i.e. followed by a `/` or the end of the pattern).                                                  |
| `[ab]`  | Matches one of the characters contained in the brackets. Character ranges, e.g. `[a-z]` are also supported. Use `[!ab]` or `[^ab]` to match any character _except_ those contained in the brackets. |
| `{a,b}` | Matches one of the patterns contained in the braces. Any of the wildcard characters can be used in the sub-patterns. Braces may be nested up to 10 levels deep.                                     |
| `!`     | When at the start of the glob, this negates the result. Multiple `!` characters negate the glob multiple times.                                                                                     |
| `\`     | A backslash character may be used to escape any of the above special characters.                                                                                                                    |

## Benchmark

### Test Case 1

```rust
const GLOB: &'static str = "some/**/n*d[k-m]e?txt";
const PATH: &'static str = "some/a/bigger/path/to/the/crazy/needle.txt";
```

```
mine                       time:   [83.742 ns 84.400 ns 85.132 ns]
glob                       time:   [386.77 ns 396.19 ns 406.87 ns]
globset                    time:   [21.010 µs 21.114 µs 21.225 µs]
glob_match                 time:   [195.58 ns 196.80 ns 198.24 ns]
glob_pre_compiled          time:   [88.180 ns 90.274 ns 92.158 ns]
globset_pre_compiled       time:   [42.680 ns 42.778 ns 42.911 ns]
```

### Test Case 2

```rust
const GLOB: &'static str = "some/**/{tob,crazy}/?*.{png,txt}";
const PATH: &'static str = "some/a/bigger/path/to/the/crazy/needle.txt";
```

```
mine                       time:   [309.80 ns 311.56 ns 313.25 ns]
globset                    time:   [31.456 µs 31.502 µs 31.559 µs]
glob_match                 time:   [384.21 ns 384.71 ns 385.15 ns]
globset_pre_compiled       time:   [42.505 ns 42.526 ns 42.551 ns]
```

## FAQ

### Why not use the more efficient `glob_match` for brace expansion?

`glob_match` is unable to handle complex brace expansions. Below are some failed examples:

- `glob_match("{a/b,a/b/c}/c", "a/b/c")`
- `glob_match("**/foo{bar,b*z}", "foobuzz")`
- `glob_match("**/{a,b}/c.png", "some/a/b/c.png")`

Due to these limitations, `brace expansion` requires a different implementation that can handle the complexity of such patterns, resulting in some performance trade-offs.


## Credits

- The [glob-match](https://github.com/devongovett/glob-match) project created by [@devongovett](https://github.com/devongovett) which is an extremely fast glob matching library in Rust.