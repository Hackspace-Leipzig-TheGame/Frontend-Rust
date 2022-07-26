use yew::{html, function_component};

const EMOJIS: &[&str] = &[
    "🫠🥹",
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
];

#[function_component(EmojiSidebar)]
pub fn emoji_sidebar() -> Html {
    let entries = EMOJIS.into_iter().map(|emoji| html!{
        <p class="text-4xl">{ emoji }</p>
    });
    html! {
        <div class="flex flex-col emoji-fontified m-2 p-2 overflow-y-scroll">
            { for entries }
        </div>
    }
}
