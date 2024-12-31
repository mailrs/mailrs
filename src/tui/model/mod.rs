#[derive(Debug)]
pub struct MBox {
    pub query: String,
    pub messages: Vec<Message>,
}

impl MBox {
    pub(crate) fn new(query: String, messages: Vec<Message>) -> Self {
        Self { query, messages }
    }
}

#[derive(Debug)]
pub struct Message {
    pub id: String,
    pub from: Option<String>,
    pub subject: Option<String>,
    pub tags: Vec<Tag>,
    // pub body: String,
}

#[derive(Debug)]
pub struct Tag {
    pub name: String,
}
