use slint;
use webbrowser;

slint::include_modules!();

fn main() {
    let sox = SteamOptionX::new().unwrap();

    sox.global::<LinkHandler>().on_link_clicked(|url| {
        let _ = webbrowser::open(url.as_str());
    });

    sox.run().unwrap();
}
