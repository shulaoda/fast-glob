use glob_match::glob_match;

#[cfg(test)]
mod tests {
  use std::path::is_separator;

use super::glob_match;

  #[test]
  fn main() {
    print!("{}", is_separator('\\'));
  }

  #[test]
  fn basic() {
    assert!(glob_match("a/**/", "a/"));
    assert!(glob_match("*", "abc"));
    assert!(glob_match("*", ""));
    assert!(glob_match("**", ""));
    assert!(glob_match("*c", "abc"));
    assert!(!glob_match("*b", "abc"));
    assert!(glob_match("a*", "abc"));
    assert!(!glob_match("b*", "abc"));
    assert!(glob_match("a*", "a"));
    assert!(glob_match("*a", "a"));
    assert!(glob_match("a*b*c*d*e*", "axbxcxdxe"));
    assert!(glob_match("a*b*c*d*e*", "axbxcxdxexxx"));
    assert!(glob_match("a*b?c*x", "abxbbxdbxebxczzx"));
    assert!(!glob_match("a*b?c*x", "abxbbxdbxebxczzy"));

    assert!(glob_match("a/*/test", "a/foo/test"));
    assert!(!glob_match("a/*/test", "a/foo/bar/test"));
    assert!(glob_match("a/**/test", "a/foo/test"));
    assert!(glob_match("a/**/test", "a/foo/bar/test"));
    assert!(glob_match("a/**/b/c", "a/foo/bar/b/c"));
    assert!(glob_match("a\\*b", "a*b"));
    assert!(!glob_match("a\\*b", "axb"));

    assert!(glob_match("[abc]", "a"));
    assert!(glob_match("[abc]", "b"));
    assert!(glob_match("[abc]", "c"));
    assert!(!glob_match("[abc]", "d"));
    assert!(glob_match("x[abc]x", "xax"));
    assert!(glob_match("x[abc]x", "xbx"));
    assert!(glob_match("x[abc]x", "xcx"));
    assert!(!glob_match("x[abc]x", "xdx"));
    assert!(!glob_match("x[abc]x", "xay"));
    assert!(glob_match("[?]", "?"));
    assert!(!glob_match("[?]", "a"));
    assert!(glob_match("[*]", "*"));
    assert!(!glob_match("[*]", "a"));

    assert!(glob_match("[a-cx]", "a"));
    assert!(glob_match("[a-cx]", "b"));
    assert!(glob_match("[a-cx]", "c"));
    assert!(!glob_match("[a-cx]", "d"));
    assert!(glob_match("[a-cx]", "x"));

    assert!(!glob_match("[^abc]", "a"));
    assert!(!glob_match("[^abc]", "b"));
    assert!(!glob_match("[^abc]", "c"));
    assert!(glob_match("[^abc]", "d"));
    assert!(!glob_match("[!abc]", "a"));
    assert!(!glob_match("[!abc]", "b"));
    assert!(!glob_match("[!abc]", "c"));
    assert!(glob_match("[!abc]", "d"));
    assert!(glob_match("[\\!]", "!"));

    assert!(glob_match("a*b*[cy]*d*e*", "axbxcxdxexxx"));
    assert!(glob_match("a*b*[cy]*d*e*", "axbxyxdxexxx"));
    assert!(glob_match("a*b*[cy]*d*e*", "axbxxxyxdxexxx"));

    assert!(glob_match("test.{jpg,png}", "test.jpg"));
    assert!(glob_match("test.{jpg,png}", "test.png"));
    assert!(glob_match("test.{j*g,p*g}", "test.jpg"));
    assert!(glob_match("test.{j*g,p*g}", "test.jpxxxg"));
    assert!(glob_match("test.{j*g,p*g}", "test.jxg"));
    assert!(!glob_match("test.{j*g,p*g}", "test.jnt"));
    assert!(glob_match("test.{j*g,j*c}", "test.jnc"));
    assert!(glob_match("test.{jpg,p*g}", "test.png"));
    assert!(glob_match("test.{jpg,p*g}", "test.pxg"));
    assert!(!glob_match("test.{jpg,p*g}", "test.pnt"));
    assert!(glob_match("test.{jpeg,png}", "test.jpeg"));
    assert!(!glob_match("test.{jpeg,png}", "test.jpg"));
    assert!(glob_match("test.{jpeg,png}", "test.png"));
    assert!(glob_match("test.{jp\\,g,png}", "test.jp,g"));
    assert!(!glob_match("test.{jp\\,g,png}", "test.jxg"));
    assert!(glob_match("test/{foo,bar}/baz", "test/foo/baz"));
    assert!(glob_match("test/{foo,bar}/baz", "test/bar/baz"));
    assert!(!glob_match("test/{foo,bar}/baz", "test/baz/baz"));
    assert!(glob_match("test/{foo*,bar*}/baz", "test/foooooo/baz"));
    assert!(glob_match("test/{foo*,bar*}/baz", "test/barrrrr/baz"));
    assert!(glob_match("test/{*foo,*bar}/baz", "test/xxxxfoo/baz"));
    assert!(glob_match("test/{*foo,*bar}/baz", "test/xxxxbar/baz"));
    assert!(glob_match("test/{foo/**,bar}/baz", "test/bar/baz"));
    assert!(!glob_match("test/{foo/**,bar}/baz", "test/bar/test/baz"));

    assert!(!glob_match("*.txt", "some/big/path/to/the/needle.txt"));
    assert!(glob_match(
      "some/**/needle.{js,tsx,mdx,ts,jsx,txt}",
      "some/a/bigger/path/to/the/crazy/needle.txt"
    ));
    assert!(glob_match(
      "some/**/{a,b,c}/**/needle.txt",
      "some/foo/a/bigger/path/to/the/crazy/needle.txt"
    ));
    assert!(!glob_match(
      "some/**/{a,b,c}/**/needle.txt",
      "some/foo/d/bigger/path/to/the/crazy/needle.txt"
    ));
    assert!(glob_match("a/{a{a,b},b}", "a/aa"));
    assert!(glob_match("a/{a{a,b},b}", "a/ab"));
    assert!(!glob_match("a/{a{a,b},b}", "a/ac"));
    assert!(glob_match("a/{a{a,b},b}", "a/b"));
    assert!(!glob_match("a/{a{a,b},b}", "a/c"));
    assert!(glob_match("a/{b,c[}]*}", "a/b"));
    assert!(glob_match("a/{b,c[}]*}", "a/c}xx"));
  }
}
