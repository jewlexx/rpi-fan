use gloo_timers::callback::Interval;
use reqwasm::http::Request;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let temp = use_state(|| 0_i128);

    let toggle_fan = |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let new_state = Request::patch("/api/fan/toggle")
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            println!("Set fan to {}", new_state);
        });
    };

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
                        .parse::<i128>()
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
        <main>
            <h1>{ "Welcome back to your Raspberry Pi" }</h1>
            <desc>{ "Its current temperature is: " } { *temp / 1000 } { " Celsius" }</desc>
            <button onclick={toggle_fan}>{ "Toggle Fan" }</button>
        </main>
    }
}
