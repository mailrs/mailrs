use std::sync::Arc;

use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::style::Stylize;
use ratatui::widgets::Block;
use ratatui::widgets::StatefulWidget;
use ratatui::widgets::Tabs;
use ratatui::widgets::Widget;
use tui_widget_list::ListBuilder;
use tui_widget_list::ListState;
use tui_widget_list::ListView;

use crate::tui::model::MBox;
use crate::tui::widgets::message_list_item::MessageListItem;

#[derive(Debug)]
pub struct Boxes {
    boxes: Vec<Arc<MBox>>,
    tab_bar_focus: usize,
    current_box_list_state: ListState,
}

impl Boxes {
    pub fn new(initial_box: MBox) -> Self {
        let boxes = vec![Arc::new(initial_box)];
        Self {
            boxes,
            tab_bar_focus: 0,
            current_box_list_state: ListState::default(),
        }
    }

    #[inline]
    pub fn focus_next(&mut self) {
        self.tab_bar_focus = self.tab_bar_focus.saturating_add(1);

        if self.tab_bar_focus >= self.boxes.len() {
            self.tab_bar_focus = self.boxes.len() - 1;
        }
    }

    #[inline]
    pub fn focus_prev(&mut self) {
        self.tab_bar_focus = self.tab_bar_focus.saturating_sub(1);
    }
}

#[derive(Debug, Default)]
pub struct BoxesState {}

impl StatefulWidget for &mut Boxes {
    type State = BoxesState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) where
        Self: Sized,
    {
        let [tab_bar, message_list] =
            Layout::vertical([Constraint::Length(3), Constraint::Percentage(100)]).areas(area);

        {
            let tabs = Tabs::new(
                self.boxes
                    .iter()
                    .map(|bx| bx.query.to_string())
                    .collect::<Vec<String>>(),
            )
            .block(Block::bordered().title("Boxes"))
            .style(Style::default().white())
            .highlight_style(Style::default().yellow())
            .select(self.tab_bar_focus)
            .divider(ratatui::symbols::DOT)
            .padding("->", "<-");

            tabs.render(tab_bar, buf);
        }

        {
            let bx = self.boxes.get(self.tab_bar_focus).unwrap().clone();

            let builder = ListBuilder::new(move |context| {
                let message = bx.messages.get(context.index).unwrap();
                let mut item = MessageListItem {
                    id: message.id.clone(),
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

            let item_count = 2;
            let list = ListView::new(builder, item_count);
            list.render(message_list, buf, &mut self.current_box_list_state);
        }
    }
}
