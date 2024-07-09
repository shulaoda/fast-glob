# fast-glob

## Introduce

A high-performance glob matching crate for Rust, originally forked from [`devongovett/glob-match`](https://github.com/devongovett/glob-match). Since the original repository hasn't been maintained for some time, I will continue to maintain it until the issues has been addressed.

**Key Features:**

* Up to 60% performance improvement.
* Support for more complex and efficient brace expansion.
* Fixed matching issues with wildcard and globstar [`glob-match/issues#9`](https://github.com/devongovett/glob-match/issues/9).

## Examples

### Simple Match

Note that simple matching does not support `brace expansion`, but all other syntaxes do.

```rust
use fast_glob::glob_match;

let glob = "some/**/*ne[d-f]dle?txt";
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
| `[ab]`  | Matches one of the characters contained in the brackets. Character ranges, e.g. `[a-z]` are also supported. Use `[!ab]` or `[^ab]` to match any character *except* those contained in the brackets. |
| `{a,b}` | Matches one of the patterns contained in the braces. Any of the wildcard characters can be used in the sub-patterns. Braces may be nested up to 10 levels deep.                                     |
| `!`     | When at the start of the glob, this negates the result. Multiple `!` characters negate the glob multiple times.                                                                                     |
| `\`     | A backslash character may be used to escape any of the above special characters.                                                                                                                    |

## Benchmark

### Test Case 1

```rust
const GLOB: &'static str = "some/**/needle.txt";
const PATH: &'static str = "some/a/bigger/path/to/the/crazy/needle.txt";
```

```
mine                       time:   [- µs - µs - µs]
glob                       time:   [- µs - µs - µs]
globset                    time:   [- ns - ns - ns]
glob_match                 time:   [- ns - ns - ns]
```

### Test Case 2

```rust
const GLOB: &'static str = "some/**/{the,crazy}/?*.{png,txt}";
const PATH: &'static str = "some/a/bigger/path/to/the/crazy/needle.txt";
```

```
mine                       time:   [- ns - ns - ns]
globset                    time:   [- ns - ns - ns]
glob_match                 time:   [- ns - ns - ns]
```

## Q\&A

### Why not use the more efficient `glob_match` for brace expansion?

`glob_match` is unable to handle complex brace expansions. Below are some failed examples:

* `glob_match("{a/b,a/b/c}/c", "a/b/c")`
* `glob_match("**/foo{bar,b*z}", "foobuzz")`
* `glob_match("**/{a,b}/c.png", "some/a/b/c.png")`

Due to these limitations, `brace expansion` requires a different implementation that can handle the complexity of such patterns, resulting in some performance trade-offs.
