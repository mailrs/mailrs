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
    raw_content: Option<String>,
}

impl Body {
    pub fn new(content: Option<String>) -> Self {
        if let Some(content) = content.as_ref() {
            let message = match mail_parser::MessageParser::default()
                .parse(file_content)
                .map_err(crate::error::Error::from)
            {
                Err(error) => {
                    tracing::warn!(?error, "Failed to parse message");
                    None
                }
                Ok(message) => Some(message),
            };
        }

        Self {
            raw_content: content,
        }
    }

    pub fn raw_content_lines(&self) -> usize {
        self.raw_content
            .as_ref()
            .map(|c| c.lines().count())
            .unwrap_or(0)
    }
}
