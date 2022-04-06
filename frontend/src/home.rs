use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let value = use_state(|| 0);

    let onclickadd = {
        let value = value.clone();
        Callback::from(move |_| value.set(*value + 1))
    };

    html! {
        <div>
            <input type="text" value="2" id="PrimeNumber" />
            <button onclick={onclickadd}>{ "+1" }</button>
            <p>{ *value }</p>
        </div>
    }
}