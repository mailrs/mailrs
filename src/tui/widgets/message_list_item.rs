use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::style::Style;
use ratatui::style::Stylize;
use ratatui::text::Text;
use ratatui::widgets::Widget;

pub struct MessageListItem {
    pub subject: Option<String>,
    pub from: Option<String>,
    pub tags: Vec<String>,
    pub style: Style,
}

impl Widget for MessageListItem {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let [from_area, subject_area, tag_list] = Layout::horizontal([
            Constraint::Percentage(30),
            Constraint::Percentage(50),
            Constraint::Length(25),
        ])
        .areas(area);

        if let Some(subject) = self.subject {
            Text::from(subject)
                .style(self.style)
                .render(subject_area, buf)
        } else {
            Text::from("<no subject>")
                .style(self.style)
                .render(subject_area, buf)
        };

        if let Some(from) = self.from {
            Text::from(from).style(self.style).render(from_area, buf)
        } else {
            Text::from("<no from>")
                .style(self.style)
                .render(from_area, buf)
        };

        Text::from(self.tags.join(", "))
            .style(self.style.italic())
            .render(tag_list, buf);
    }
}
