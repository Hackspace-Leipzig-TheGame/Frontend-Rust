use yew::{html, function_component};
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    #[prop_or(1)]
    pub number: i8
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div class="bg-red-50 border-4 border-red-400 shadow-inner rounded-md w-48 h-64">
           {props.number}
           <img src={format!("https://assets.pokemon.com/assets/cms2/img/pokedex/full/{:0>3}.png", props.number)}/>
        </div>
    }
}
