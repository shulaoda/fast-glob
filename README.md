# fast-glob

## Introduce

A high-performance glob matching crate for Rust, originally forked from [`devongovett/glob-match`](https://github.com/devongovett/glob-match). Since the original repository hasn't been maintained for some time, I will continue to maintain it until the issues has been addressed.

**Key Features:**

- Up to 60% performance improvement.
- Support for more complex and efficient brace expansion.
- Fixed matching issues with wildcard and globstar [`glob-match/issues#9`](https://github.com/devongovett/glob-match/issues/9).

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
mine                       time:   [63.347 ns 63.389 ns 63.450 ns]
glob                       time:   [380.53 ns 382.89 ns 386.44 ns]
globset                    time:   [27.686 µs 27.696 µs 27.707 µs]
glob_match                 time:   [197.79 ns 198.67 ns 199.79 ns]
```

### Test Case 2

```rust
const GLOB: &'static str = "some/**/{tob,crazy}/?*.{png,txt}";
const PATH: &'static str = "some/a/bigger/path/to/the/crazy/needle.txt";
```

```
mine                       time:   [403.06 ns 405.17 ns 407.76 ns]
globset                    time:   [36.929 µs 37.284 µs 37.731 µs]
glob_match                 time:   [367.10 ns 369.04 ns 371.61 ns]
```

## Q\&A

### Why not use the more efficient `glob_match` for brace expansion?

`glob_match` is unable to handle complex brace expansions. Below are some failed examples:

- `glob_match("{a/b,a/b/c}/c", "a/b/c")`
- `glob_match("**/foo{bar,b*z}", "foobuzz")`
- `glob_match("**/{a,b}/c.png", "some/a/b/c.png")`

Due to these limitations, `brace expansion` requires a different implementation that can handle the complexity of such patterns, resulting in some performance trade-offs.
