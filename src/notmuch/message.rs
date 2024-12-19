#[derive(Debug)]
pub struct Message {
    id: String,
    #[allow(unused)]
    date: i64, // TODO
}

impl Message {
    pub fn new(id: String, date: i64) -> Self {
        Self { id, date }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}
