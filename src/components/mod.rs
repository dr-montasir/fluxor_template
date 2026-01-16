mod head;
mod header;
mod badge;
mod nav;
mod logo;
mod footer;
mod scripts;
mod notfound;
mod layout;

pub use head::head;
pub use header::header;
pub use badge::badge;
pub use footer::footer;
pub use scripts::SW_REGISTER;
pub use layout::{LAYOUT_TEMPLATE, layout};
pub use notfound::not_found_page;