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
pub struct BoxesState {
    boxes: Vec<crate::widgets::mbox::MBoxState>,
    tab_bar_focus: usize,
}

impl BoxesState {
    pub fn new(bx: Arc<crate::model::MBox>) -> Self {
        Self {
            boxes: vec![crate::widgets::mbox::MBoxState::new(bx)],
            tab_bar_focus: 0,
        }
    }

    pub fn add_box(&mut self, bx: Arc<crate::model::MBox>) {
        self.boxes.push(crate::widgets::mbox::MBox::new(bx));
    }

    pub fn remove_index(&mut self, i: usize) {
        self.boxes.remove(i);
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

    #[inline]
    pub fn focus_last(&mut self) {
        self.tab_bar_focus = self.boxes.len() - 1;
    }

    pub fn get_current_state_mut(&mut self) -> Option<&mut super::mbox::MBoxState> {
        self.boxes.get_mut(self.tab_bar_focus)
    }

    pub(crate) fn remove_current_box(&mut self) {
        self.boxes.remove(self.tab_bar_focus);
        self.focus_prev();
    }

    pub(crate) fn current_index(&self) -> usize {
        self.tab_bar_focus
    }
}

#[derive(Debug)]
pub struct Boxes;

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
                state
                    .boxes
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

        if let Some(mbox_state) = state.get_current_state_mut() {
            tracing::debug!("Rendering box");
            let mut mbox = crate::widgets::mbox::MBox;
            mbox.render(message_list, buf, mbox_state);
        } else {
            tracing::debug!("Rendering no box, none there");
        }
    }
}
