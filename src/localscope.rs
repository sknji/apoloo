pub struct LocalScope {
    pub local_count: i8,
    pub scope_depth: i8,
    pub locals: Vec<Local>,
}

pub struct Local {
    name: String,
    depth: i8,
}

impl Local {
    pub fn new(name: String, depth: i8) -> Self {
        Self { name, depth }
    }
}

impl LocalScope {
    pub fn new() -> Self {
        Self { local_count: 0, locals: Vec::new(), scope_depth: 0 }
    }

    pub fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    pub fn end_scope(&mut self) -> u8 {
        self.scope_depth -= 1;
        let mut counts = 0;
        while self.local_count > 0 && self.locals.get((self.local_count - 1) as usize).unwrap().depth > self.scope_depth
        {
            counts += 1;
            self.local_count -= 1
        }
        counts
    }

    pub fn add_local(&mut self, name: &String) {
        self.local_count += 1;
        let local = Local::new(name.to_string(), self.scope_depth);
        self.locals.push(local);
    }

    pub fn contains(&self, name: &str) -> bool {
        let mut counter = self.local_count - 1;

        while counter >= 0 {
            let l = self.locals.get(counter as usize).unwrap();
            if l.depth != -1 && l.depth < self.scope_depth {
                break;
            }

            if l.name.eq(name) {
                return true;
            }

            counter -= 1;
        }

        false
    }

    pub fn resolve_local(&self, name: &str) -> Option<u8> {
        let mut counter = self.local_count - 1;

        while counter >= 0 {
            let l = self.locals.get(counter as usize);
            match l {
                None => {}
                Some(v) => {
                    if v.name.eq(name) {
                        return Some(counter as u8);
                    }
                }
            }

            counter -= 1;
        }

        return None;
    }
}
