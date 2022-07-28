use yew::{html, function_component};

const EMOJIS: &[&str] = &[
    "🥹",
    "🥳",
    "🥲",
    "🤯",
    "🦩",
    "🤬",
    "🤌",
    "️❤️",
    "️🔥",
    "✌️",
    "🍆",
    "🍑",
    "🐒",
    "🐮",
    "👀",
    "👋",
    "👍",
    "👎",
    "👿",
    "💀",
    "💢",
    "💯",
    "😇",
    "😓",
    "😡",
    "😢",
    "😭",
    "😰",
    "😱",
    "🤙",
    "🤜",
    "🤣",
    "🤦",
    "🫠",
];

#[function_component(EmojiSidebar)]
pub fn emoji_sidebar() -> Html {
    let entries = EMOJIS.into_iter().map(|emoji| html!{
        <p class="text-4xl p-1 m-1 border border-neutral-200 select-none rounded cursor-pointer hover:shadow hover:border-neutral-500">{ emoji }</p>
    });
    html! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 emoji-fontified m-2 p-2 overflow-y-scroll">
            { for entries }
        </div>
    }
}
