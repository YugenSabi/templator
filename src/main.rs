mod cli;
mod generator;

use dialoguer::console::style;

fn run() -> Result<(), std::io::Error> {
    let args = cli::parse()?;
    generator::run(&args.template, &args.destination)?;
    println!(
        "\n{} Проект создан: {}",
        style("✔").green().bold(),
        style(args.destination.display()).cyan().bold()
    );
    Ok(())
}

fn main() {
    match run() {
        Ok(()) => {}
        Err(error) => {
            eprintln!("\n{} {error}", style("Ошибка:").red().bold());
            std::process::exit(1)
        }
    }
}
