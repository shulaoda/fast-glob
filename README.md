# fast-glob

## Introduce

An extremely fast glob matching library which is forked from [`devongovett/glob-match`](https://github.com/devongovett/glob-match). Due to the author hasn't maintained the repository for a long time, I will continue to maintain it until the issues has been responded.

* Nearly 60% performance improvement
* Inheriting the excellent features of [`glob-match`](https://github.com/devongovett/glob-match)
* Fixed matching issues with wildcard and globstar [`glob-match/issues#9`](https://github.com/devongovett/glob-match/issues/9)

⚠️ There are some breaking changes in this crate, please check in this [PR#15](https://github.com/devongovett/glob-match/pull/15)

## Example

```rust
use fast_glob::glob_match;

assert!(glob_match("some/**/{a,b,c}/**/needle.txt", "some/path/a/to/the/needle.txt"));
```

Wildcard values can also be captured using the `glob_match_with_captures` function. This returns a `Vec` containing ranges within the path string that matched dynamic parts of the glob pattern. You can use these ranges to get slices from the original path string.

```rust
use fast_glob::glob_match_with_captures;

let glob = "some/**/{a,b,c}/**/needle.txt";
let path = "some/path/a/to/the/needle.txt";
let result = glob_match_with_captures(glob, path)
  .map(|v| v.into_iter().map(|capture| &path[capture]).collect());

assert_eq!(result, Some(vec!["path", "a", "to/the"]));
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

## Bench

```
globset                 time:   [24.429 µs 24.522 µs 24.676 µs]
glob                    time:   [335.71 ns 338.09 ns 341.18 ns]
fast_glob               time:   [78.030 ns 78.237 ns 78.475 ns]
```