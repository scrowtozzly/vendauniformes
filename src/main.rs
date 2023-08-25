use stylist::style;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(App)]
fn app() -> Html {
    let tab_style = style!(
        r#"
            display: flex;
            align-items: center;
            justify-content: center;
        "#,
    )
    .unwrap();

    html! {
        <div class={tab_style}>
            <button> {"CLIENTE"} </button>
            <button> {"FUNCIONARIO"} </button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
