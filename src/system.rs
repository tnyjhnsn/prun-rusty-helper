use yew::prelude::*;
use yew::services::ConsoleService;

use crate::models::Universe;
use crate::planet_details::PlanetDetails;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub universe: Universe,
    pub env_filter: bool,
}

#[allow(dead_code)]
pub struct System {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    OnHeadingClick(String),
}

impl Component for System {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnHeadingClick(_s) => {
                // TODO
                ConsoleService::info(&format!("Feature not implimented yet"));
            }
        }
        false
    }

    fn view(&self) -> Html {
        if let Some(selected_star) = &self.props.universe.selected_star {
            let heading = format!("Planets/Resources in the {} System",
                                  selected_star.name);
            let loading = selected_star.name.eq(&"".to_string());
            html! {
                <div hidden={loading}>
                    <h3>{heading}</h3>
                    { for self.props.universe.planets_for_selected_star()
                        .iter().map(|p| {
                            let resources = self.props.universe.resources_for_planet(p);
                            html! {
                                <PlanetDetails
                                    planet={p}
                                    resources={resources}
                                    highlight_env=true
                                    env_filter={self.props.env_filter}
                                    universe={&self.props.universe}
                                    heading_click=self.link.callback(Msg::OnHeadingClick)
                                />
                            }
                        })
                    }
                </div>
            }
        } else {
            html! {<></>}
        }
    }
}
