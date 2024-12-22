use crate::tui::model::MBox;

#[derive(Debug)]
pub struct Boxes {
    boxes: Vec<MBox>,
}

impl Boxes {
    pub fn new(initial_box: MBox) -> Self {
        let boxes = vec![initial_box];
        Self { boxes }
    }
}

#[derive(Debug, Default)]
pub struct BoxesState {}

impl ratatui::widgets::StatefulWidget for &Boxes {
    type State = BoxesState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) where
        Self: Sized,
    {
    }
}
