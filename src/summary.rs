use yew::prelude::*;

use crate::models::{Planet, Resource, Universe};
use crate::planet_details::PlanetDetails;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub universe: Universe,
    pub env_filter: bool,
}

#[allow(dead_code)]
pub struct Summary {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for Summary {
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
        let selected_res = &self.props.universe.selected_res;
        let diagnostics = &self.props.universe.diagnostics;
        let summary = if selected_res.eq("- None -") { format!("Filter: {} Planets, {} Star Systems",
            diagnostics.planets_with_env,
            diagnostics.stars_with_planets_with_env_res)}
        else {format!("Filter: {} Planets, {} Star Systems with {} resources",
            diagnostics.planets_with_env_res(),
            diagnostics.stars_with_planets_with_env_res,
            selected_res)
        };
        let top_hits: Vec::<(Planet, Resource)> = diagnostics.filter_hits
            .iter()
            .take(8)
            .cloned()
            .collect();
        let title = format!("Top {} hits from the filter", top_hits.len());

        html! {
            <div class="diagnostics">
                <div style="margin: '0.5rem';">
                    {summary}
                </div>
                <h3>{if top_hits.len() > 0 {title} else {"".to_string()}}</h3>
                { for top_hits
                    .into_iter()
                    .map(|(p, r)| {
                        let resources = vec![r];
                        html! {
                            <PlanetDetails
                                planet={p}
                                resources={resources}
                                highlight_env=false
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

