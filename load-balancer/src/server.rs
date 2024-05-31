#[derive(Debug, Clone)]
pub struct Server {
    pub addr: String,
    pub id: i32,
    pub active: bool,
}

impl Server {
    pub fn new(addr: &str, id: i32) -> Self {
        Server {
            addr: addr.to_string(),
            id: id,
            active: true,
        }
    }
    pub fn enable(&mut self) {
        self.active = true;
    }

    pub fn disable(&mut self) {
        self.active = false;
    }
}
