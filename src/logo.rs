use yew::prelude::*;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub is_loading: bool,
}

#[allow(dead_code)]
pub struct Logo {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for Logo {
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
        let loading_text = match self.props.is_loading {
            true => "Calculating Star and Planet data. Please wait...",
            _=> "",
        };

        html! {
            <>
                <div class="logo">
                    <h1>{"Plexucra"}<span class="cc">{".de"}</span></h1>
                    <span class="byline">{"Tools for the Galactic CEO"}</span>
                </div>
                <h3>{loading_text}</h3>
            </>
        }
    }
}
