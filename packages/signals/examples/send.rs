use dioxus::prelude::*;
use dioxus_signals::*;

fn main() {
    // dioxus::desktop::launch(App);
}

#[component]
fn App() -> Element {
    let mut signal = use_signal_sync(|| 0);

    use_hook(|| {
        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            signal += 1;
        });
    });

    rsx! {
        button { onclick: move |_| signal += 1, "Increase" }
        "{signal}"
    }
}
