use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyEvent;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::text::Text;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::List;
use ratatui::widgets::ListDirection;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

#[derive(Default)]
pub struct CommanderUi {
    activated: bool,
    pub unknown_command: bool,
    pub suggestions: Vec<String>,
    input: Input,
}

impl CommanderUi {
    pub fn clear(&mut self) {
        self.input.reset();
        self.activated = false;
        self.unknown_command = false;
    }

    pub fn activate(&mut self) {
        self.activated = true;
    }

    pub fn is_activated(&self) -> bool {
        self.activated
    }

    pub fn handle_key_press(&mut self, event: KeyEvent) {
        self.input.handle_event(&Event::Key(event));
    }

    pub fn value(&self) -> &str {
        self.input.value()
    }
}

impl Widget for &CommanderUi {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        if self.activated {
            let msg = vec![
                Span::styled("ESC", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit prompt"),
            ];
            let style = Style::default().add_modifier(Modifier::RAPID_BLINK);

            let [suggestions_area, command_area] =
                Layout::vertical([Constraint::Min(3), Constraint::Min(3)]).areas(area);

            if !self.suggestions.is_empty() {
                List::new(self.suggestions.clone())
                    .block(Block::bordered().title("Suggestions"))
                    .style(Style::new().white())
                    .highlight_style(Style::new().italic())
                    .highlight_symbol(">>")
                    .repeat_highlight_symbol(true)
                    .direction(ListDirection::BottomToTop)
                    .render(suggestions_area, buf);
            }

            let [commander_logo_area, inserting_area, desc_area] = Layout::horizontal([
                Constraint::Min(3),
                Constraint::Percentage(100),
                Constraint::Min(20),
            ])
            .areas(command_area);

            let logo = Paragraph::new(
                Text::from(Line::from(Span::styled(
                    ":",
                    Style::default().add_modifier(Modifier::BOLD),
                )))
                .style(style),
            )
            .block(Block::default().borders(Borders::ALL));

            let desc_text = Paragraph::new(Text::from(Line::from(msg)).style(style))
                .block(Block::default().borders(Borders::ALL));

            let input = Paragraph::new(self.input.value())
                .style(if self.unknown_command {
                    Style::default().on_red()
                } else {
                    Style::default()
                })
                .block(Block::default().borders(Borders::ALL).title("Input"));

            logo.render(commander_logo_area, buf);
            input.render(inserting_area, buf);
            desc_text.render(desc_area, buf);
        }
    }
}
