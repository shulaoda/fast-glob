use std::path::is_separator;

#[derive(Clone, Copy, Debug, Default)]
struct State {
  // These store character indices into the glob and path strings.
  path_index: usize,
  glob_index: usize,

  // When we hit a * or **, we store the state for backtracking.
  wildcard: Wildcard,
}

#[derive(Clone, Copy, Debug, Default)]
struct Wildcard {
  // Using u32 rather than usize for these results in 10% faster performance.
  glob_index: u32,
  path_index: u32,
}

#[derive(PartialEq)]
enum BraceState {
  Invalid,
  Comma,
  EndBrace,
}

struct BraceStack {
  stack: [State; 10],
  length: u32,
  longest_brace_match: u32,
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
          // Backtrack
          state.wildcard.glob_index = state.glob_index as u32;
          state.wildcard.path_index = state.path_index as u32 + 1;

          state.glob_index += 1;

          if state.path_index < path.len() && is_separator(path[state.path_index] as char) {
            state.wildcard.path_index = 0;
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
        b'[' if state.path_index < path.len() => {
          state.glob_index += 1;
          let c = path[state.path_index];

          // Check if the character class is negated.
          let mut negated = false;
          if state.glob_index < glob.len() && matches!(glob[state.glob_index], b'^' | b'!') {
            negated = true;
            state.glob_index += 1;
          }

          // Try each range.
          let mut first = true;
          let mut is_match = false;
          while state.glob_index < glob.len() && (first || glob[state.glob_index] != b']') {
            let mut low = glob[state.glob_index];
            if !unescape(&mut low, glob, &mut state.glob_index) {
              // Invalid pattern!
              return false;
            }
            state.glob_index += 1;

            // If there is a - and the following character is not ], read the range end character.
            let high = if state.glob_index + 1 < glob.len()
              && glob[state.glob_index] == b'-'
              && glob[state.glob_index + 1] != b']'
            {
              state.glob_index += 1;
              let mut high = glob[state.glob_index];
              if !unescape(&mut high, glob, &mut state.glob_index) {
                // Invalid pattern!
                return false;
              }
              state.glob_index += 1;
              high
            } else {
              low
            };

            if low <= c && c <= high {
              is_match = true;
            }
            first = false;
          }

          if state.glob_index >= glob.len() {
            // invalid pattern!
            return false;
          }

          state.glob_index += 1;
          if is_match != negated {
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
fn unescape(c: &mut u8, glob: &[u8], glob_index: &mut usize) -> bool {
  if *c == b'\\' {
    *glob_index += 1;
    if *glob_index >= glob.len() {
      // Invalid pattern!
      return false;
    }
    *c = match glob[*glob_index] {
      b'a' => b'\x61',
      b'b' => b'\x08',
      b'n' => b'\n',
      b'r' => b'\r',
      b't' => b'\t',
      c => c,
    }
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

impl Default for BraceStack {
  #[inline]
  fn default() -> Self {
    // Manual implementation is faster than the automatically derived one.
    BraceStack {
      stack: [State::default(); 10],
      length: 0,
      longest_brace_match: 0,
    }
  }
}

impl BraceStack {
  #[inline(always)]
  fn push(&mut self, state: &State) -> State {
    // Push old state to the stack, and reset current state.
    self.stack[self.length as usize] = *state;
    self.length += 1;
    State {
      path_index: state.path_index,
      glob_index: state.glob_index + 1,
      ..State::default()
    }
  }

  #[inline(always)]
  fn pop(&mut self, state: &State) -> State {
    self.length -= 1;

    self.stack[self.length as usize].glob_index = state.glob_index;
    self.stack[self.length as usize].path_index = self.longest_brace_match as usize;

    if self.length == 0 {
      self.longest_brace_match = 0;
    }

    self.stack[self.length as usize]
  }

  #[inline(always)]
  fn last(&self) -> &State {
    &self.stack[self.length as usize - 1]
  }
}
