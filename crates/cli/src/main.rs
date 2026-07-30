mod app;
use app::App;

fn main() {
    let app = App::new();

    match ratatui::run(|terminal| app.run(terminal)) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{e:#}");
            std::process::exit(1);
        }
    }
}
