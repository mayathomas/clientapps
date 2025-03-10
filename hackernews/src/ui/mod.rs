mod comments;
mod stories;
use dioxus::prelude::*;
use comments::Comments;
use stories::Stories;

use crate::StoryData;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Debug, Clone)]
pub enum CommentsState {
    Unset,
    Loading,
    Loaded(StoryData),
}

#[component]
pub fn App() -> Element {
    use_context_provider(|| Signal::new(CommentsState::Unset));
    rsx! {
        document::Link{rel:"icon", href: FAVICON},
        document::Link{rel:"stylesheet", href: TAILWIND_CSS},
        main { class: "flex w-full h-full shadow-lg rounded-3xl",
        section { class: "flex flex-col w-4/12 h-full pt-3 overflow-y-scroll bg-gray-50",
            Stories {  }
        }
        section { class: "flex flex-col w-8/12 px-4 bg-white rounded-r-3xl",
            section {
                Comments{}
            }
        }
    }
    }
}

