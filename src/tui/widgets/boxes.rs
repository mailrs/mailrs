use std::sync::Arc;

use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::style::Style;
use ratatui::style::Stylize;
use ratatui::widgets::Block;
use ratatui::widgets::StatefulWidget;
use ratatui::widgets::Tabs;
use ratatui::widgets::Widget;

#[derive(Debug)]
pub struct Boxes {
    boxes: Vec<crate::tui::widgets::mbox::MBox>,
}

impl Boxes {
    pub fn empty() -> Self {
        Self { boxes: Vec::new() }
    }

    pub fn add_box(&mut self, bx: Arc<crate::tui::model::MBox>) {
        self.boxes.push(crate::tui::widgets::mbox::MBox::new(bx));
    }

    pub fn remove_index(&mut self, i: usize) {
        self.boxes.remove(i);
    }
}

#[derive(Debug, Default)]
pub struct BoxesState {
    tab_bar_focus: usize,
    box_state: Vec<super::mbox::MBoxState>,
}

impl BoxesState {
    #[inline]
    pub fn focus_next(&mut self) {
        self.tab_bar_focus = self.tab_bar_focus.saturating_add(1);

        if self.tab_bar_focus >= self.box_state.len() {
            self.tab_bar_focus = self.box_state.len() - 1;
        }
    }

    #[inline]
    pub fn focus_prev(&mut self) {
        self.tab_bar_focus = self.tab_bar_focus.saturating_sub(1);
    }

    #[inline]
    pub fn focus_last(&mut self) {
        self.tab_bar_focus = self.box_state.len() - 1;
    }

    pub fn get_current_state_mut(&mut self) -> Option<&mut super::mbox::MBoxState> {
        self.box_state.get_mut(self.tab_bar_focus)
    }

    pub(crate) fn increase_boxes_count(&mut self) {
        self.box_state.push(Default::default());
    }

    pub(crate) fn decrease_boxes_count(&mut self) {
        self.box_state.remove(self.tab_bar_focus);
        self.focus_prev();
    }

    pub(crate) fn current_index(&self) -> usize {
        self.tab_bar_focus
    }
}

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
                    .map(|bx| bx.query().to_string())
                    .collect::<Vec<String>>(),
            )
            .block(Block::bordered().title("Boxes"))
            .style(Style::default().white())
            .highlight_style(Style::default().yellow())
            .select(state.tab_bar_focus)
            .divider(ratatui::symbols::DOT)
            .padding("->", "<-");

            tabs.render(tab_bar, buf);
        }

        if let Some((bx, state)) = self
            .boxes
            .get_mut(state.tab_bar_focus)
            .into_iter()
            .zip(state.get_current_state_mut())
            .next()
        {
            bx.render(message_list, buf, state);
        } else {
            tracing::debug!("Rendering no box, none there");
        }
    }
}
