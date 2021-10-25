use yew::prelude::*;

use crate::models::Toggle;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub show_routes: bool,
    pub toggle_signal: Callback<(Toggle, bool)>,
}

pub enum Msg {
    OnChange,
}

#[allow(dead_code)]
pub struct ShowRoutes {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for ShowRoutes {
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
            Msg::OnChange => {
                let show = !self.props.show_routes;
                self.props.toggle_signal.emit((Toggle::ShowRoutes, show));
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <label for="show-routes">
                    <input
                        type="checkbox"
                        name="show-croutesx"
                        checked=self.props.show_routes
                        onchange=self.link.callback(|_| Msg::OnChange)
                    />
                {"Show Routes"}
                </label>
            </div>
        }
    }
}

