use std::sync::Arc;

use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::StatefulWidget;
use tui_widget_list::ListBuilder;
use tui_widget_list::ListState;
use tui_widget_list::ListView;

use super::message_list_item::MessageListItem;

#[derive(Debug)]
pub struct MBoxState {
    mbox: Arc<crate::model::MBox>,
    list_state: ListState,
    show_message: Option<(String, super::message::MessageState)>,
}

impl MBoxState {
    pub fn new(boxes: Arc<crate::model::MBox>) -> Self {
        Self {
            mbox: boxes,
            list_state: ListState::default(),
            show_message: None,
        }
    }

    pub fn query(&self) -> &str {
        &self.mbox.query
    }

    pub fn show_current_message(&mut self) {
        let message = self
            .list_state
            .selected
            .and_then(|index| {
                tracing::debug!(index, "Try to find message by index");
                self.mbox.messages.get(index)
            })
            .map(|message| {
                tracing::debug!(?message, "Found message");
                let message_id = message.id.clone();
                let state = super::message::MessageState {
                    body_state: super::message::BodyState::new({
                        let l = message.body.lines();

                        if l > u16::MAX as usize {
                            tracing::warn!("Body longer than {}", u16::MAX);
                            u16::MAX
                        } else {
                            l as u16
                        }
                    }),
                };

                (message_id, state)
            });

        tracing::info!(?message, "Setting message to show");
        self.show_message = message;
    }

    pub fn currently_shown_message_mut(&mut self) -> Option<&mut super::message::MessageState> {
        self.show_message.as_mut().map(|(_, m)| m)
    }

    #[inline]
    pub fn next(&mut self) {
        self.list_state.next()
    }

    #[inline]
    pub fn prev(&mut self) {
        self.list_state.previous()
    }

    pub fn hide_message(&mut self) {
        self.show_message = None;
    }
}

#[derive(Debug)]
pub struct MBox;

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
        if let Some((message_id, message_state)) = state.show_message.as_mut() {
            tracing::info!(?message_id, "Showing message");

            let Some(message) = state.mbox.messages.iter().find(|m| m.id == *message_id) else {
                tracing::warn!(
                    message_id = message_id,
                    "Trying to show message by id, but did not find that message!"
                );
                // TODO
                return;
            };

            let mut message = crate::widgets::message::Message::from(message);
            message.render(area, buf, message_state);
        } else {
            tracing::info!("Showing message box");

            let item_count = state.mbox.messages.len();

            let boxes = state.mbox.clone();
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
}
