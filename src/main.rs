use leptos::view;
use tracing::*;
use derive_ui::*;

fn main() {
	derive_ui::init_debug_tools();

	leptos::mount_to_body(move |cx| view! {cx, <GenericViewer data="123".to_owned() />})
}
