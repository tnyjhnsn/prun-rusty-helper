use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub star_list: Vec<String>,
    pub search_star_signal: Callback<String>,
}

pub enum Msg {
    SubmitSearch(ChangeData)
}

#[allow(dead_code)]
pub struct StarSearch {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for StarSearch {
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
            Msg::SubmitSearch(e) => {
                if let ChangeData::Value(select) = e {
                    self.props.search_star_signal.emit(select);
                }
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="search-input">
                <label for="search">{"Star search"}</label>
                <input
                    class="search"
                    list="star-list"
                    name="search"
                    onchange=self.link.callback(Msg::SubmitSearch)
                />
                <datalist id="star-list">
                { for self.props.star_list.iter().map(|v| {
                    html! { <option value={v.to_string()}>{v}</option> }
                })}
                </datalist>
            </div>
        }
    }
}

