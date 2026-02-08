use rest_api_tui::tui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tui::run_app()?;
    Ok(())
}
