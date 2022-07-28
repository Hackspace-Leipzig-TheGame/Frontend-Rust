use yew::Properties;
use yew::{function_component, html};

#[derive(Properties, PartialEq)]
pub struct CardProps {
    #[prop_or(1)]
    pub number: i8,
    #[prop_or(true)]
    pub showNumber: bool,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let imageURL = format!(
        "https://assets.pokemon.com/assets/cms2/img/pokedex/full/{:0>3}.png",
        props.number
    );
    html! {
        <div
            class="bg-red-50 shadow-inner w-48 h-64
            border-4 border-[#ff0000] rounded-md 
            flex flex-col justify-around 
            text-center"
        >
            if props.showNumber {
                <p class="font-bold text-lg">{props.number}</p>
            }
            <img src={imageURL}/>
        </div>
    }
}
