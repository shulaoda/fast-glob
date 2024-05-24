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

  // Store the state when we see an opening '{' brace in a stack.
  // Up to 10 nested braces are supported.
  let mut brace_stack = BraceStack::default();

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

          // If the next char is a special brace separator,
          // skip to the end of the braces so we don't try to match it.
          if brace_stack.length > 0
            && state.glob_index < glob.len()
            && matches!(glob[state.glob_index], b',' | b'}')
          {
            if state.skip_braces(glob, false) == BraceState::Invalid {
              // invalid pattern!
              return false;
            }
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
          if !is_separator(path[state.path_index] as char) {
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
        }
        b'{' => {
          if brace_stack.length as usize >= brace_stack.stack.len() {
            // Invalid pattern! Too many nested braces.
            return false;
          }

          // Push old state to the stack, and reset current state.
          state = brace_stack.push(&state);
          continue;
        }
        b'}' if brace_stack.length > 0 => {
          // If we hit the end of the braces, we matched the last option.
          brace_stack.longest_brace_match =
            brace_stack.longest_brace_match.max(state.path_index as u32);

          state.glob_index += 1;
          state = brace_stack.pop(&state);
          continue;
        }
        b',' if brace_stack.length > 0 => {
          // If we hit a comma, we matched one of the options!
          // But we still need to check the others in case there is a longer match.
          brace_stack.longest_brace_match =
            brace_stack.longest_brace_match.max(state.path_index as u32);

          // restore
          state.wildcard.path_index = 0;
          state.glob_index = state.glob_index + 1;
          state.path_index = brace_stack.last().path_index;
          continue;
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

    if brace_stack.length > 0 {
      // If in braces, find next option and reset path to index where we saw the '{'
      match state.skip_braces(glob, true) {
        BraceState::Comma => {
          state.path_index = brace_stack.last().path_index;
          continue;
        }
        BraceState::Invalid => {
          return false;
        }
        BraceState::EndBrace => {
          // Hit the end. Pop the stack.
          // If we matched a previous option, use that.
          if brace_stack.longest_brace_match > 0 {
            state = brace_stack.pop(&state);
            continue;
          } else {
            // Didn't match. Restore state, and check if we need to jump back to a star pattern.
            state = *brace_stack.last();
            brace_stack.length -= 1;

            if state.wildcard.path_index > 0 && state.wildcard.path_index as usize <= path.len() {
              state.backtrack();
              continue;
            }
          }
        }
      }
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

  fn skip_braces(&mut self, glob: &[u8], stop_on_comma: bool) -> BraceState {
    let mut braces = 1;
    let mut in_brackets = false;

    while self.glob_index < glob.len() && braces > 0 {
      match glob[self.glob_index] {
        // Skip nested braces.
        b'{' if !in_brackets => braces += 1,
        b'}' if !in_brackets => braces -= 1,
        b',' if stop_on_comma && braces == 1 && !in_brackets => {
          self.glob_index += 1;
          return BraceState::Comma;
        }
        c @ (b'*' | b'[') if !in_brackets => {
          if c == b'[' {
            in_brackets = true;
          }
          if c == b'*' {
            if self.glob_index + 1 < glob.len() && glob[self.glob_index + 1] == b'*' {
              self.glob_index += 1;
            }
          }
        }
        b']' => in_brackets = false,
        b'\\' => self.glob_index += 1,
        _ => {}
      }

      self.glob_index += 1;
    }

    if braces != 0 {
      return BraceState::Invalid;
    }

    BraceState::EndBrace
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
