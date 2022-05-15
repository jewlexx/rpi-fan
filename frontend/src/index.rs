use gloo_timers::callback::Interval;
use reqwasm::http::Request;
use yew::prelude::*;

#[derive(Debug, Clone, Copy)]
enum StateMsg {
    SetTemp(f32),
    SetState(bool),
}

#[function_component]
pub fn Index() -> Html {
    let temp = use_state(|| 0.);

    {
        let temp = temp.clone();
        use_effect(move || {
            let interval = Interval::new(1000u32, move || {
                let temp = temp.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let new_temp = Request::get("/api/temp")
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap()
                        .parse::<f32>()
                        .unwrap();

                    temp.set(new_temp);
                });
            });

            || {
                interval.cancel();
            }
        });
    }

    html! {
        <div class="root">
            <h1>{ "Welcome back to your Raspberry Pi" }</h1>
            <desc>{ "Its current temperature is: " } { *temp } { " Celsius" }</desc>
        </div>
    }
}
