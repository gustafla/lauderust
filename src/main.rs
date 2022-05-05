use std::{collections::HashMap, time::Duration};
use futures_util::StreamExt;
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
    log::info!("a");
    {
        log::info!("b");
        let users = users.clone();
        use_effect_with_deps(
            move |_| {
                log::info!("c");
                let users = users.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    log::info!("d");
                    let fetched: Vec<User> = Request::get("https://hackathlon.nitorio.us/users")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    users.set(fetched);

                    let users_vec = Request::get("https://hackathlon.nitorio.us/users")
                        .send()
                        .await
                        .unwrap()
                        .json::<Vec<User>>()
                        .await
                        .unwrap();
                    let users = users_vec
                        .into_iter()
                        .map(|user| (user.id.clone(), user))
                        .collect::<HashMap<String, User>>();

                    let mut old_user_locations;
                    let mut user_locations =
                        Request::get("https://hackathlon.nitorio.us/coordinates")
                            .send()
                            .await
                            .unwrap()
                            .json::<Vec<UserLocation>>()
                            .await
                            .unwrap();

                    let interval = Duration::from_secs(1);

                    let mut ticker =
                        gloo_timers::future::IntervalStream::new(interval.as_millis() as u32);

                    loop {
                        ticker.next().await;

                        old_user_locations = user_locations;
                        user_locations = Request::get("https://hackathlon.nitorio.us/coordinates")
                            .send()
                            .await
                            .unwrap()
                            .json::<Vec<UserLocation>>()
                            .await
                            .unwrap();

                        log::info!("");

                        for (loc, old_loc) in
                            std::iter::zip(user_locations.iter(), old_user_locations.iter())
                        {
                            let earth_radius = 6371000.0;

                            let distance = earth_radius
                                * f64::acos(
                                    f64::sin(loc.coordinates.lat.to_radians())
                                        * f64::sin(old_loc.coordinates.lat.to_radians())
                                        + f64::cos(loc.coordinates.lat.to_radians())
                                            * f64::cos(old_loc.coordinates.lat.to_radians())
                                            * f64::cos(
                                                loc.coordinates.long.to_radians()
                                                    - old_loc.coordinates.long.to_radians(),
                                            ),
                                );

                            log::info!(
                                "{}: speed {} m/s",
                                users[&loc.user_id].name,
                                distance / interval.as_secs_f64()
                            );
                        }
                    }
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
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
