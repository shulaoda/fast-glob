use std::path::is_separator;

#[derive(Clone, Copy, Debug, Default)]
struct State {
  // These store character indices into the glob and path strings.
  path_index: usize,
  glob_index: usize,

  // When we hit a * or **, we store the state for backtracking.
  wildcard: Wildcard,
  globstar: Wildcard,
}

#[derive(Clone, Copy, Debug, Default)]
struct Wildcard {
  // Using u32 rather than usize for these results in 10% faster performance.
  glob_index: u32,
  path_index: u32,
}

pub fn glob_match(glob: &str, path: &str) -> bool {
  glob_match_internal(glob, path)
}

fn glob_match_internal<'a>(glob: &str, path: &'a str) -> bool {
  // This algorithm is based on https://research.swtch.com/glob
  let glob = glob.as_bytes();
  let path = path.as_bytes();

  let mut state = State::default();

  // First, check if the pattern is negated with a leading '!' character.
  // Multiple negations can occur.
  let mut negated = false;
  while state.glob_index < glob.len() && glob[state.glob_index] == b'!' {
    negated = !negated;
    state.glob_index += 1;
  }

  while state.glob_index < glob.len() || state.path_index < path.len() {
    if state.glob_index < glob.len() {
      match glob[state.glob_index] {
        b'*' => {
          let is_globstar = state.glob_index + 1 < glob.len() && glob[state.glob_index + 1] == b'*';

          if is_globstar {
            // Coalesce multiple ** segments into one.
            state.glob_index = skip_globstars(glob, state.glob_index);
          }

          // If mismatch, restart in next path index.
          state.wildcard.glob_index = state.glob_index as u32;
          state.wildcard.path_index = state.path_index as u32 + 1;

          // ** allows path separators, whereas * does not.
          // However, ** must be a full path component, i.e. a/**/b not a**b.
          if is_globstar {
            state.glob_index += 2;

            if state.glob_index < glob.len() && glob[state.glob_index] == b'/' {
              state.glob_index += 1;
            }

            state.globstar = state.wildcard;
          } else {
            state.glob_index += 1;
          }

          continue;
        }
        b'?' if state.path_index < path.len() => {
          if !is_separator(path[state.path_index] as char) {
            state.glob_index += 1;
            state.path_index += 1;
            continue;
          }
        }
        mut c if state.path_index < path.len() => {
          // Match escaped characters as literals.
          if !unescape(&mut c, glob, &mut state.glob_index) {
            // Invalid pattern!
            return false;
          }

          let is_match = if c == b'/' {
            is_separator(path[state.path_index] as char)
          } else {
            path[state.path_index] == c
          };

          if is_match {
            state.glob_index += 1;
            state.path_index += 1;

            if c == b'/' {
              state.wildcard.path_index = 0;
            }

            continue;
          }
        }

        _ => {}
      }
    }

    if state.path_index < path.len()
      && is_separator(path[state.path_index] as char)
      && state.globstar.path_index != state.wildcard.path_index
    {
      // Special case: don't jump back for a / at the end of the glob.
      if state.globstar.path_index > 0 && state.path_index + 1 < path.len() {
        state.wildcard.glob_index = state.globstar.glob_index;
      } else {
        state.wildcard.path_index = 0;
      }
    }

    // If we didn't match, restore state to the previous star pattern.
    if state.wildcard.path_index > 0 && state.wildcard.path_index as usize <= path.len() {
      state.backtrack();
      continue;
    }

    return negated;
  }

  !negated
}

#[inline(always)]
fn skip_globstars(glob: &[u8], mut glob_index: usize) -> usize {
  glob_index += 2;
  // Coalesce multiple ** segments into one.
  while glob_index + 3 <= glob.len()
    && unsafe { glob.get_unchecked(glob_index..glob_index + 3) } == b"/**"
  {
    glob_index += 3;
  }
  glob_index - 2
}

#[inline(always)]
fn unescape(c: &mut u8, glob: &[u8], glob_index: &mut usize) -> bool {
  if *c == b'\\' {
    *glob_index += 1;
    if *glob_index >= glob.len() {
      // Invalid pattern!
      return false;
    }
    *c = glob[*glob_index];
  }
  true
}

impl State {
  #[inline(always)]
  fn backtrack(&mut self) {
    self.glob_index = self.wildcard.glob_index as usize;
    self.path_index = self.wildcard.path_index as usize;
  }
}
