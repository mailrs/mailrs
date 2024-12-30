use std::sync::Arc;

use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::StatefulWidget;
use tui_widget_list::ListBuilder;
use tui_widget_list::ListState;
use tui_widget_list::ListView;

use super::message_list_item::MessageListItem;

#[derive(Debug)]
pub struct MBox {
    boxes: Arc<crate::tui::model::MBox>,
}

impl MBox {
    pub fn new(boxes: Arc<crate::tui::model::MBox>) -> Self {
        Self { boxes }
    }

    pub fn query(&self) -> &str {
        &self.boxes.query
    }
}

#[derive(Debug, Default)]
pub struct MBoxState {
    list_state: ListState,
}

impl StatefulWidget for &mut MBox {
    type State = MBoxState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) where
        Self: Sized,
    {
        let item_count = self.boxes.messages.len();

        let boxes = self.boxes.clone();
        let builder = ListBuilder::new(move |context| {
            let message = boxes.messages.get(context.index).unwrap();
            let mut item = MessageListItem {
                from: message.from.clone(),
                subject: message.subject.clone(),
                tags: message.tags.iter().map(|t| t.name.to_string()).collect(),
                style: Style::default(),
            };

            // Alternating styles
            if context.index % 2 == 0 {
                item.style = Style::default().bg(Color::Rgb(28, 28, 32));
            } else {
                item.style = Style::default().bg(Color::Rgb(0, 0, 0));
            }

            // Style the selected element
            if context.is_selected {
                item.style = Style::default()
                    .bg(Color::Rgb(255, 153, 0))
                    .fg(Color::Rgb(28, 28, 32));
            };

            // Return the size of the widget along the main axis.
            let main_axis_size = 1;

            (item, main_axis_size)
        });

        let list = ListView::new(builder, item_count);
        list.render(area, buf, &mut state.list_state);
    }
}
