use yew::prelude::*;

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

impl Component for System {
    type Message = ();
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

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let selected_star = &self.props.universe.selected_star;
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
                            />
                        }
                    })
                }
            </div>
        }
    }
}
