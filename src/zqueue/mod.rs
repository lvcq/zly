pub struct Queue<T> {
  q_data: Vec<T>,
}

impl<T> Queue<T> {
  pub fn new() -> Self {
    Queue { q_data: Vec::new() }
  }

  pub fn push(&mut self, item: T) {
    self.q_data.push(item);
  }

  pub fn pop(&mut self) -> Option<T> {
    let len = self.q_data.len();
    if len > 0 {
      let item = self.q_data.remove(0);
      return Some(item);
    } else {
      return None;
    }
  }

  pub fn is_empty(&self) -> bool {
    return self.q_data.len() == 0;
  }
}
