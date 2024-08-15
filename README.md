# fast-glob

## Introduce

A high-performance glob matching crate for Rust based on [`devongovett/glob-match`](https://github.com/devongovett/glob-match). 

**Key Features:**

- Up to 60% performance improvement.
- Supports more complete and well-rounded features.

## Examples

### Simple Match

Note that simple matching does not support `brace expansion`, but all other syntaxes do.

```rust
use fast_glob::glob_match;

let glob = "some/**/n*d[k-m]e?txt";
let path = "some/a/bigger/path/to/the/crazy/needle.txt";

assert!(glob_match(glob, path));
```

### Brace Expansion

Brace expansion is supported by using `glob_match_with_brace`. While the performance is lower than simple match, some performance loss is inevitable due to the complexity of brace expansion.

```rust
use fast_glob::glob_match_with_brace;

let glob = "some/**/{the,crazy}/?*.{png,txt}";
let path = "some/a/bigger/path/to/the/crazy/needle.txt";

assert!(glob_match_with_brace(glob, path));
```

### Multi-Pattern Matching

You can build a matcher like `globset` and add multiple patterns to match.

```rust
use fast_glob::Glob;

// let mut glob = Glob::new(glob);
let mut glob = Glob::default();

assert!(glob.add("*.txt"));
assert!(glob.is_match("name.txt"));
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
mine                       time:   [75.860 ns 76.625 ns 77.468 ns]
glob                       time:   [369.87 ns 376.28 ns 383.55 ns]
globset                    time:   [21.077 µs 21.234 µs 21.458 µs]
glob_match                 time:   [192.68 ns 193.82 ns 195.02 ns]
glob_pre_compiled          time:   [86.975 ns 87.593 ns 88.182 ns]
globset_pre_compiled       time:   [42.360 ns 42.574 ns 42.803 ns]
```

### Test Case 2

```rust
const GLOB: &'static str = "some/**/{tob,crazy}/?*.{png,txt}";
const PATH: &'static str = "some/a/bigger/path/to/the/crazy/needle.txt";
```

```
mine                       time:   [487.75 ns 491.41 ns 496.32 ns]
globset                    time:   [31.717 µs 31.857 µs 32.062 µs]
glob_match                 time:   [391.12 ns 394.70 ns 399.05 ns]
globset_pre_compiled       time:   [42.726 ns 42.828 ns 42.954 ns]
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