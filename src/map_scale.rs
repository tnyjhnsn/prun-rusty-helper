use yew::prelude::*;

use crate::models::ScaleOptions;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub scale_options: ScaleOptions,
    pub set_scale_signal: Callback<f64>,
}

pub enum Msg {
    OnChange(ChangeData),
}

#[allow(dead_code)]
pub struct MapScale {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for MapScale {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
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
            Msg::OnChange(cd) => {
                if let ChangeData::Select(select) = cd {
                    let v = select.value().parse::<f64>().unwrap();
                    self.props.set_scale_signal.emit(v);
                } else {
                    ();
                }
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
            <label for="map-size">
                {"Map Size"}
                <select
                    name="map-size"
                    onchange=self.link.callback(Msg::OnChange)
                >
                    { for self.props.scale_options.iter().map(|(k, v)| {
                        html! {
                            <option value={v.0.to_string()} selected={v.1}>{k}</option>
                        }
                    })}
                </select>
            </label>
            </div>
        }
    }
}

