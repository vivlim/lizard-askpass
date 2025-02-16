use std::{
    fs,
    io::{stderr, Read},
};

use color_eyre::Result;
use lizard_askpass_common::interactive::run;

use futures::StreamExt;
use ratatui::{
    layout::Rect,
    prelude::{CrosstermBackend, TermionBackend},
    termion::{get_tty, raw::IntoRawMode, screen::IntoAlternateScreen},
    Terminal, TerminalOptions,
};
use systemd_ask_password_agent;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = systemd_ask_password_agent::system_requests()?;
    while let Some(r) = stream.next().await {
        eprintln!("Handling a request");
        eprint!("{}: ", r.message);
        //let tty = get_tty()?;
        let tty = fs::File::create("/dev/console")?;
        let tty_read = fs::File::open("/dev/console")?;
        eprintln!("opened tty");
        let writer = tty.try_clone()?.into_raw_mode()?.into_alternate_screen()?;
        eprintln!("switched to raw mode & alternate screen");
        let mut backend = TermionBackend::new(writer);
        let options: TerminalOptions = TerminalOptions {
            viewport: ratatui::Viewport::Fixed(Rect::new(0, 0, 80, 24)),
        };
        let mut terminal = Terminal::with_options(backend, options)?;
        terminal.clear()?;
        eprintln!("running interactive keyboard");
        let text = run(terminal, tty_read)?;
        let x = text;
        let res = r.reply(&x).await;
        if let Err(e) = res {
            eprintln!("Error replying: {e}");
        }
    }
    return Ok(());
}
