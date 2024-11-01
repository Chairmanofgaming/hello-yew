use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1> {"Hello World, I made this in rust."}</h1>
       
        
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
    }
