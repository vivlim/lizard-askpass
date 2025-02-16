use color_eyre::Result;
use lizard_askpass_common::interactive::run;
use ratatui::{prelude::CrosstermBackend, DefaultTerminal, Terminal};

fn main() -> Result<()> {
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend);
    let input = std::io::stdin();
    let text = run(terminal?, input)?;

    println!("{}", &text);

    Ok(())
}
