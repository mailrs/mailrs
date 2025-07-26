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
    pub body: Body,
}

#[derive(Debug)]
pub struct Tag {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Body {
    pub(crate) content: Option<String>,
}

impl Body {
    pub fn new(content: Option<String>) -> Self {
        Self { content }
    }

    pub fn lines(&self) -> usize {
        self.content
            .as_ref()
            .map(|c| c.lines().count())
            .unwrap_or(0)
    }
}
