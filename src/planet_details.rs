use yew::prelude::*;
use crate::models::{Universe, Planet, Resource};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub planet: Planet,
    pub resources: Vec<Resource>,
    pub highlight_env: bool,
    pub env_filter: bool,
    pub universe: Universe,
}

#[allow(dead_code)]
pub struct PlanetDetails {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for PlanetDetails {
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
        let p = &self.props.planet;
        let p_filtered = self.props.env_filter && p.filtered;
        let css_surface = if p.surface {"fa-mountain"} else {"fa-wind"};
        let css_filtered = if self.props.highlight_env && p_filtered {"filtered"} else {""};
        let class = format!("fas {} {}", css_surface, css_filtered);
        let res_max_factor = &self.props.universe.res_max_factor;
        let selected_res = &self.props.universe.selected_res;
        let highlight_env = self.props.highlight_env;

        html! {
            <div class="icon-heading">
                <i class={class} />
                <h4 class=classes!("heading", css_filtered)>
                    {&self.props.planet.name}
                </h4>
                <ul class="base-build" hidden=true></ul>
                <ul>{for self.props.resources
                        .iter()
                        .map(|r| get_res_li(
                            r,
                            res_max_factor.get(&r.ticker).unwrap(),
                            highlight_env && r.ticker.eq(selected_res)
                        ))
                    }
                </ul>
            </div>
        }
    }
}

fn get_res_li(res: &Resource, max_factor: &f64, filtered: bool) -> Html {
    let typ = format!("{}{}", (&res.typ[..1].to_string()).to_uppercase(), &res.typ[1..]);
    let conc = res.factor/max_factor;
    let colour = match conc {
        c if c >= 0.66 => "conc-high",
        c if c >= 0.33 => "conc-medium",
        _=> "conc-low",
    };
    let v1 = (conc * 100.0).round() as i32;
    let v2 = (res.factor * 100.0).round() as i32;
    let v3 = (max_factor * 100.0).round() as i32;

    let f = if filtered {"filtered"} else {""};
    let ticker = format!("{} ({})", res.ticker, typ);
    let ratio = format!(" {}% ({}/{})", v1, v2, v3);

    html! {
        <li class={f}>{ticker}<span class={colour}> {ratio}</span></li>
    }
}
