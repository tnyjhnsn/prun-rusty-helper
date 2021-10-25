use yew::prelude::*;

use crate::models::Toggle;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub show_cx: bool,
    pub toggle_signal: Callback<(Toggle, bool)>,
}

pub enum Msg {
    OnChange,
}

#[allow(dead_code)]
pub struct ShowCx {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for ShowCx {
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
                let show = !self.props.show_cx;
                self.props.toggle_signal.emit((Toggle::ShowCx, show));
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <label for="show-cx">
                    <input
                        type="checkbox"
                        name="show-cx"
                        checked=self.props.show_cx
                        onchange=self.link.callback(|_| Msg::OnChange)
                    />
                {"Show CXs"}
                </label>
            </div>
        }
    }
}

