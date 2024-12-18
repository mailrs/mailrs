use crate::cli::Cli;
use crate::config::Config;
use crate::error::ApplicationError;

slint::include_modules!();

pub(crate) async fn start(cli: Cli, config: Config) -> Result<(), ApplicationError> {
    let startup_query = cli.init_query.unwrap_or(config.default_query);
    tracing::debug!(?startup_query);
    match cli.mode {
        crate::cli::Mode::Gui => {
            let ui_result = tokio::task::spawn_blocking(|| {
                let ui = AppWindow::new()?;

                ui.on_request_increase_value({
                    let ui_handle = ui.as_weak();
                    move || {
                        let ui = ui_handle.unwrap();
                        ui.set_counter(ui.get_counter() + 1);
                    }
                });

                ui.run()
            });

            let _ = ui_result.await?;
        }
        crate::cli::Mode::Tui => {
            eprintln!("TUI mode is not implemented yet!");
            std::process::exit(1);
        }
    }

    Ok(())
}
