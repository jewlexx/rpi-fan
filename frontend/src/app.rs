use gloo_timers::callback::Interval;
use yew::prelude::*;

fn create_client() -> reqwest::Client {
    reqwest::Client::new()
}

#[function_component(App)]
pub fn app() -> Html {
    let temp = use_state(|| 0_i128);

    let toggle_fan = |_| {
        let client = create_client();
        wasm_bindgen_futures::spawn_local(async move {
            let new_state = client
                .clone()
                .patch("http://127.0.0.1:8000/api/fan/toggle")
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
                    let client = create_client();
                    let new_temp = client
                        .get("http://127.0.0.1:8000/api/temp")
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
