use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::Borders;
use tui_logger::TuiLoggerLevelOutput;
use tui_logger::TuiLoggerWidget;
use tui_logger::TuiWidgetState;

#[derive(Default)]
pub struct Logger;

pub struct LoggerState {
    state: TuiWidgetState,
}

impl LoggerState {
    pub fn new() -> Self {
        Self {
            state: TuiWidgetState::new().set_default_display_level(log::LevelFilter::Debug),
        }
    }
}

impl ratatui::widgets::StatefulWidget for Logger {
    type State = LoggerState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let w = TuiLoggerWidget::default()
            .block(
                Block::default()
                    .title("mailrs logs")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style_error(Style::default().fg(Color::Red))
            .style_debug(Style::default().fg(Color::Green))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_trace(Style::default().fg(Color::Magenta))
            .style_info(Style::default().fg(Color::Cyan))
            .output_separator(':')
            .output_timestamp(Some("%H:%M:%S".to_string()))
            .output_level(Some(TuiLoggerLevelOutput::Long))
            .output_target(false)
            .output_file(true)
            .output_line(true)
            .state(&state.state);

        ratatui::widgets::Widget::render(w, area, buf);
    }
}
