use reqwasm::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
struct User {
    id: String,
    name: String,
    email: String,
    from_flight_id: String,
    to_flight_id: String,
    image_url: String,
    activity_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Flight {
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Location {
    lat: f64,
    long: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct UserLocation {
    user_id: String,
    coordinates: Location,
}

#[derive(Clone, Properties, PartialEq)]
struct UserListProps {
    users: Vec<User>,
}

#[function_component(UserList)]
fn user_list(UserListProps { users }: &UserListProps) -> Html {
    users
        .iter()
        .map(|user| {
            html! {<p>{format!("{:#?}", user)}</p>}
        })
        .collect()
}

#[function_component(App)]
fn app() -> Html {
    let users = use_state(|| vec![]);
    {
        let users = users.clone();
        use_effect_with_deps(
            move |_| {
                let users = users.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched: Vec<User> = Request::get("https://hackathlon.nitorio.us/users")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    users.set(fetched);
                });
                || ()
            },
            (),
        );
    }
    html! {
        <div>
        <h1>{ "Users" }</h1>
        <UserList users={(*users).clone()} />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
