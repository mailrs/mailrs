use ratatui::layout::Alignment;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::style::Style;
use ratatui::style::Stylize;
use ratatui::widgets::Block;
use ratatui::widgets::Cell;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Row;
use ratatui::widgets::StatefulWidget;
use ratatui::widgets::Table;
use ratatui::widgets::Wrap;

#[derive(Debug)]
pub struct Message {
    header: MessageHeader,
    text: String,
}

#[derive(Debug)]
struct MessageHeader {
    pub id: String,
    pub from: Option<String>,
    pub subject: Option<String>,
    pub tags: Vec<String>,
}

impl From<&crate::model::Message> for Message {
    fn from(value: &crate::model::Message) -> Self {
        Self {
            header: MessageHeader {
                id: value.id.clone(),
                from: value.from.clone(),
                subject: value.subject.clone(),
                tags: value.tags.iter().map(|t| t.name.clone()).collect(),
            },

            text: String::new(), // TODO
        }
    }
}

#[derive(Debug)]
pub struct MessageState {}

impl StatefulWidget for &mut MessageHeader {
    type State = MessageState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _state: &mut Self::State,
    ) {
        let rows = [
            Row::new(vec![Cell::from("ID"), Cell::from(self.id.to_string())]),
            Row::new(vec![Cell::from("From"), {
                if let Some(from) = self.from.as_ref() {
                    Cell::from(from.clone())
                } else {
                    Cell::from("no from").italic()
                }
            }]),
            Row::new(vec![Cell::from("Subject"), {
                if let Some(subject) = self.subject.as_ref() {
                    Cell::from(subject.clone())
                } else {
                    Cell::from("no subject").italic()
                }
            }]),
            Row::new(vec![Cell::from("Tags"), {
                Cell::from(self.tags.join(", "))
            }]),
        ];

        let widths = [Constraint::Length(8), Constraint::Percentage(100)];

        let table = Table::new(rows, widths)
            .block(Block::new().title("Header"))
            .style(Style::new().blue())
            .column_spacing(1);

        ratatui::widgets::Widget::render(table, area, buf);
    }
}

impl StatefulWidget for &mut Message {
    type State = MessageState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let [header_area, text_area] =
            Layout::vertical([Constraint::Min(6), Constraint::Percentage(100)]).areas(area);

        let text = Paragraph::new(self.text.clone())
            .block(Block::bordered().title("Paragraph"))
            .style(Style::new().white().on_black())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .scroll((0, 0));

        self.header.render(header_area, buf, state);
        ratatui::widgets::Widget::render(text, text_area, buf);
    }
}
