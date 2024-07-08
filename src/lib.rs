mod brace;
mod glob;

use brace::Pattern;
use glob::glob_match_normal;

#[derive(Debug, Default)]
pub struct Glob {
  glob: Vec<u8>,
  pattern: Pattern,
}

impl Glob {
  pub fn new(glob: &str) -> Option<Self> {
    let mut value = Vec::with_capacity(glob.len() + 2);
    value.push(b'{');
    value.extend(glob.as_bytes());
    value.push(b'}');

    if let Some(mut pattern) = Pattern::new(&value[1..value.len() - 1]) {
      pattern.branch.push((0, 1));
      pattern.shadow.push((0, 0));
      return Some(Glob {
        glob: value,
        pattern,
      });
    }
    None
  }

  pub fn add(&mut self, glob: &str) -> bool {
    if self.glob.len() == 0 {
      if let Some(c) = Self::new(glob) {
        *self = c;
        return true;
      }
      return false;
    }

    let glob = glob.as_bytes();
    if let Some(branch) = Pattern::parse(glob) {
      self.pattern.branch[0].1 += 1;
      self.pattern.branch.extend(branch);
      self.glob.reserve_exact(glob.len() + 1);

      let index = self.glob.len() - 1;
      self.glob[index] = b',';
      self.glob.extend(glob);
      self.glob.push(b'}');

      return true;
    }
    false
  }

  pub fn is_match(&mut self, path: &str) -> bool {
    let mut flag = false;
    loop {
      let (result, longest_index) = glob_match_normal(&self.pattern.value, path.as_bytes());
      if result || !self.pattern.trigger(&self.glob, longest_index) {
        if flag {
          self.pattern.restore();
          self.pattern.track(&self.glob);
        }
        return result;
      }
      flag = true;
    }
  }
}

pub fn glob_match(glob: &str, path: &str) -> bool {
  glob_match_normal(glob.as_bytes(), path.as_bytes()).0
}

pub fn glob_match_with_brace(glob: &str, path: &str) -> bool {
  let glob = glob.as_bytes();
  let path = path.as_bytes();

  if let Some(pattern) = &mut Pattern::new(glob) {
    loop {
      let (result, longest_index) = glob_match_normal(&pattern.value, path);

      if result || !pattern.trigger(glob, longest_index) {
        return result;
      }
    }
  }
  false
}
