use yew::prelude::*;

use crate::components::EmojiSidebar;
use crate::components::Card;

pub struct App {
    nothing: u8,
}

pub enum Msg {

}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            nothing: 0,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <main class="w-screen h-screen flex flex-col">
                <p class="text-4xl text-center">{ "this will have stuff soon" }</p>
                <div class="flex flex-row grow h-full">
                    <p class="grow shrink">{ "the game?" }</p>
                    <div>
                        <EmojiSidebar />
                    </div>
                </div>
                <div class="text-center">
                    <Card number=5 />
                </div>
            </main>
        }
    }
}
