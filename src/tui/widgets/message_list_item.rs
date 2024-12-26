use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::style::Style;
use ratatui::style::Stylize;
use ratatui::text::Text;
use ratatui::widgets::Widget;

pub struct MessageListItem {
    pub id: String,
    pub tags: Vec<String>,
    pub style: Style,
}

impl Widget for MessageListItem {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let [id, tag_list] =
            Layout::horizontal([Constraint::Percentage(80), Constraint::Length(25)]).areas(area);

        Text::from(self.id).style(self.style).render(id, buf);

        Text::from(self.tags.join(", "))
            .style(self.style.italic())
            .render(tag_list, buf);
    }
}
