use ratatui::layout::Alignment;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::style::Style;
use ratatui::style::Stylize;
use ratatui::widgets::Block;
use ratatui::widgets::Cell;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Row;
use ratatui::widgets::ScrollbarState;
use ratatui::widgets::StatefulWidget;
use ratatui::widgets::Table;
use ratatui::widgets::Widget;
use ratatui::widgets::Wrap;

#[derive(Debug)]
pub struct Message {
    header: MessageHeader,
    body: crate::model::Body,
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

            body: value.body.clone(),
        }
    }
}

#[derive(Debug)]
pub struct MessageState {
    pub(crate) body_state: BodyState,
}

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

        self.header.render(header_area, buf, state);
        Body { body: &self.body }.render(text_area, buf, &mut state.body_state);
    }
}

#[derive(Debug)]
pub struct Body<'a> {
    body: &'a crate::model::Body,
}

#[derive(Debug)]
pub struct BodyState {
    body_lines: u16,
    scrollbar: ScrollbarState,
    vertical_scroll: u16,
    horizontal_scroll: u16,
}

impl BodyState {
    pub fn new(body_lines: u16) -> Self {
        Self {
            body_lines,
            scrollbar: ScrollbarState::default(),
            vertical_scroll: 0,
            horizontal_scroll: 0,
        }
    }

    #[inline]
    pub fn scroll_to_top(&mut self) {
        tracing::debug!(body_lines = self.body_lines, "Scrolling to top");
        self.scrollbar.first();
        self.vertical_scroll = 0;
    }

    #[inline]
    pub fn scroll_to_bottom(&mut self) {
        tracing::debug!(body_lines = self.body_lines, "Scrolling to bottom");
        self.scrollbar.last();
        self.vertical_scroll = self.body_lines;
    }

    #[inline]
    pub fn scroll_down(&mut self) {
        if self.body_lines <= self.vertical_scroll {
            tracing::debug!("Not scrolling down, as there are no more lines");
            return;
        }

        tracing::debug!("Scrolling down");
        self.scrollbar.next();
        self.vertical_scroll = self.vertical_scroll.saturating_add(1);
    }

    #[inline]
    pub fn scroll_up(&mut self) {
        tracing::debug!("Scrolling up");
        self.scrollbar.prev();
        self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
    }
}

impl StatefulWidget for Body<'_> {
    type State = BodyState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) where
        Self: Sized,
    {
        if let Some(content) = self.body.content.as_ref() {
            let offset = (state.vertical_scroll, state.horizontal_scroll);
            tracing::debug!(?offset, "Displaying body");

            let text = Paragraph::new(content.clone())
                .block(Block::bordered())
                .style(Style::new().white().on_black())
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true })
                .scroll(offset);

            text.render(area, buf);
        } else {
            ratatui::text::Text::from("no body found")
                .style(Style::new().italic())
                .render(area, buf);
        }
    }
}
