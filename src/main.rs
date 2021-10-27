#![recursion_limit = "256"]

use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

mod models;
use models::*;
mod canvas;
use canvas::*;
mod logo;
use logo::*;
mod map_scale;
use map_scale::*;
mod star_search;
use star_search::*;
mod show_cx;
use show_cx::*;
mod show_routes;
use show_routes::*;
mod edit_filters;
use edit_filters::*;
mod system;
use system::*;
mod planet_details;
mod summary;
use summary::*;

// TODO sort out what should be in Universe and what should be in PrUnApp
struct PrUnApp {
    is_loading: bool,
    universe: Universe,
    map_features: MapFeatures,
    filters: Filters,
    fetch_stars: Option<FetchTask>,
    fetch_planets: Option<FetchTask>,
    fetch_resources: Option<FetchTask>,
    link: ComponentLink<Self>,
}

#[allow(dead_code)]
enum Msg {
    MakeStarReq,
    MakePlanetReq,
    MakeResourceReq,
    RespStar(Result<Vec<Star>, anyhow::Error>),
    RespPlanet(Result<Vec<Planet>, anyhow::Error>),
    RespResource(Result<Vec<Resource>, anyhow::Error>),
    SelectedStar(Star),
    SetScale(f64),
    SearchStar(String),
    Toggle((Toggle, bool)),
    Surface(SurfaceOption),
    Environment((Environment, EnvironmentOption)),
    SelectedRes(Option<String>),
    TestMe,
}

impl Component for PrUnApp {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakeStarReq);
        link.send_message(Msg::MakePlanetReq);
        link.send_message(Msg::MakeResourceReq);
        Self {
            is_loading: true,
            universe: Universe::new(),
            map_features: MapFeatures::new(),
            filters: Filters::new(),
            fetch_stars: None,
            fetch_planets: None,
            fetch_resources: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeStarReq => {
                self.is_loading = true;
                let req = Request::get("http://localhost:8081/stars.json")
                    .body(Nothing)
                    .expect("can make req");

                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<Star>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::RespStar(data)
                    },
                );

                let task = FetchService::fetch(req, cb)
                    .expect("can create task");
                self.fetch_stars = Some(task);
                return false
            }
            Msg::MakePlanetReq => {
                self.is_loading = true;
                let req = Request::get("http://localhost:8081/planets.json")
                    .body(Nothing)
                    .expect("can make req");

                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<Planet>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::RespPlanet(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_planets = Some(task);
                return false
            }
            Msg::MakeResourceReq => {
                self.is_loading = true;
                let req = Request::get("http://localhost:8081/resources.json")
                    .body(Nothing)
                    .expect("can make req");

                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<Resource>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::RespResource(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_resources = Some(task);
                return false
            }
            Msg::RespStar(resp) => {
                if let Ok(data) = resp {
                    self.universe.stars = data;
                    self.universe.fix_star_y();
                    self.is_loading = false;
                }
                return true
            }
            Msg::RespPlanet(resp) => {
                if let Ok(data) = resp {
                    self.universe.planets = data;
                    self.universe.create_star_list();
                    self.is_loading = false;
                }
                return true
            }
            Msg::RespResource(resp) => {
                if let Ok(data) = resp {
                    self.universe.resources = data;
                    self.universe.create_resource_data();
                    self.is_loading = false;
                }
                return true
            }
            Msg::SelectedStar(star) => {
                self.universe.selected_star = star;
                return true
            }
            Msg::SetScale(scale) => {
                self.map_features.set_selected_scale(scale);
                return true
            }
            Msg::SearchStar(name) => {
                if let Some(star) = self.universe.star_from_name(name) {
                    self.link.callback(Msg::SelectedStar).emit(star);
                }
                return false
            }
            Msg::Toggle((toggle, b)) => {
                match toggle {
                    Toggle::ShowCx => self.map_features.show_cx = b,
                    Toggle::ShowRoutes => self.map_features.show_routes = b,
                    Toggle::IncEnvFilter => {
                        self.filters.env_filter = b;
                        self.universe.apply_filters(&self.filters);
                    },
                    Toggle::IncNormal => {
                        self.filters.inc_normal = b;
                        self.universe.apply_filters(&self.filters);
                    },
                };
                return true
            }
            Msg::Surface(surface) => {
                self.filters.surface = surface;
                self.universe.apply_filters(&self.filters);
                return true
            }
            Msg::Environment((env, option)) => {
                match env {
                    Environment::Gravity => self.filters.gravity = option,
                    Environment::Temp => self.filters.temp = option,
                    Environment::Pressure => self.filters.pressure = option,
                }
                self.universe.apply_filters(&self.filters);
                return true
            }
            Msg::SelectedRes(res) => {
                self.universe.selected_res = res;
                self.universe.apply_filters(&self.filters);
                return true
            }
            Msg::TestMe => {
                //self.filter_editor.apply_filters(&self.universe.planets);
                return true
            }
            // put a return value @ end of each Msg
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Canvas
                    map_features=&self.map_features
                    universe=&self.universe
                    env_filter=self.filters.env_filter
                    selected_star_signal=self.link.callback(Msg::SelectedStar)
                />
                <div class="app">
                    <div class="panel1">
                        <div class="sticky">
                            <Logo is_loading=self.is_loading />
                            <MapScale
                                scale_options=self.map_features.scale_options.clone()
                                set_scale_signal=self.link.callback(Msg::SetScale)
                            />
                        </div>
                    </div>
                    <div class="panel2">
                        <div class="sticky">
                            <StarSearch
                                star_list=self.universe.star_list.clone()
                                search_star_signal=self.link.callback(Msg::SearchStar)
                            />
                        </div>
                    </div>
                    <div class="panel3">
                        <div class="sticky">
                            <ShowCx
                                show_cx=self.map_features.show_cx
                                toggle_signal=self.link.callback(Msg::Toggle)
                            />
                            <ShowRoutes
                                show_routes=self.map_features.show_routes
                                toggle_signal=self.link.callback(Msg::Toggle)
                            />
                            <EditFilters
                                env_filter=self.filters.env_filter
                                inc_normal=self.filters.inc_normal
                                res_list=self.universe.res_list.clone()
                                toggle_signal=self.link.callback(Msg::Toggle)
                                surface_signal=self.link.callback(Msg::Surface)
                                env_signal=self.link.callback(Msg::Environment)
                                selected_res_signal=self.link.callback(Msg::SelectedRes)
                            />
                            <Summary
                                universe=&self.universe
                                env_filter=self.filters.env_filter
                                search_star_signal=self.link.callback(Msg::SearchStar)
                            />
                        </div>
                    </div>
                    <div class="panel4">
                        <div class="sticky">
                            <System
                                universe=&self.universe
                                env_filter=self.filters.env_filter
                            />
                        </div>
                    </div>
                </div>
            </>
        }
    }
}

pub fn main() {
    yew::start_app::<PrUnApp>();
}
