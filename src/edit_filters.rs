use yew::{html::IntoPropValue, prelude::*};

use crate::models::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub env_filter: bool,
    pub inc_normal: bool,
    pub res_list: Vec<String>,
    pub toggle_signal: Callback<(Toggle, bool)>,
    pub surface_signal: Callback<SurfaceOption>,
    pub env_signal: Callback<(Environment, EnvironmentOption)>,
    pub selected_res_signal: Callback<String>,
}

pub enum Msg {
    OnChangeToggle(Toggle),
    OnChangeSurface(ChangeData),
    OnChangeEnvironment(Environment, ChangeData),
    OnChangeSelectedRes(ChangeData),
}

#[allow(dead_code)]
pub struct EditFilters {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for EditFilters {
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
            Msg::OnChangeToggle(toggle) => {
                if let Toggle::IncEnvFilter = toggle {
                    let v = !self.props.env_filter;
                    self.props.toggle_signal.emit((Toggle::IncEnvFilter, v));
                } else {
                    let v = !self.props.inc_normal;
                    self.props.toggle_signal.emit((Toggle::IncNormal, v));
                }
            }
            Msg::OnChangeSurface(cd) => {
                if let ChangeData::Select(select) = cd {
                    let surface = to_surface_option(&select.value());
                    self.props.surface_signal.emit(surface);
                } else {
                    ();
                }
            }
            Msg::OnChangeEnvironment(env, cd) => {
                if let ChangeData::Select(select) = cd {
                    let option = to_env_option(&select.value());
                    self.props.env_signal.emit((env, option));
                } else {
                    ();
                }
            }
            Msg::OnChangeSelectedRes(cd) => {
                if let ChangeData::Select(res) = cd {
                    self.props.selected_res_signal.emit(res.value());
                } else {
                    ();
                }
            }
        }
        false
    }

    fn view(&self) -> Html {
        let gravity_callback =
            |cd| Msg::OnChangeEnvironment(Environment::Gravity, cd);
        let temp_callback =
            |cd| Msg::OnChangeEnvironment(Environment::Temp, cd);
        let pressure_callback =
            |cd| Msg::OnChangeEnvironment(Environment::Pressure, cd);
        let env_filter_callback = |_| Msg::OnChangeToggle(Toggle::IncEnvFilter);
        let inc_normal_callback = |_| Msg::OnChangeToggle(Toggle::IncNormal);

        let rocky = SurfaceOption::Rocky.to_string();
        let gaseous = SurfaceOption::Gaseous.to_string();
        let both = SurfaceOption::Both.to_string();

        html! {
            <div>
                {checkbox("Include Environment Filter?".to_string(),
                    self.props.env_filter, self.link.callback(env_filter_callback))
                }
                <div class="environment-filter" hidden={!self.props.env_filter}>
                {"Environment Filter"}
                    <div>
                        <select
                            onchange=self.link.callback(|cd| Msg::OnChangeSurface(cd))
                        >
                            <option value={rocky.clone()}>{rocky}</option>
                            <option value={gaseous.clone()}>{gaseous}</option>
                            <option value={both.clone()}>{both}</option>
                        </select>
                        {"Surface"}
                    </div>
                    {dropdown(Position::R, "Gravity".to_string(),
                        self.link.callback(gravity_callback))
                    }
                    {dropdown(Position::R, "Temperature".to_string(),
                        self.link.callback(temp_callback))
                    }
                    {dropdown(Position::R, "Pressure".to_string(),
                        self.link.callback(pressure_callback))
                    }
                    <div class="inc-normal">
                    {checkbox("Include Normal with Low and High?".to_string(),
                        self.props.inc_normal, self.link.callback(inc_normal_callback))
                    }
                    </div>
                </div>
                <div class="resource-filter">
                    <div>
                    {"Include Resource Filter:"}
                        <select
                            onchange=self.link.callback(|cd| Msg::OnChangeSelectedRes(cd))
                        >
                        { for self.props.res_list.iter().map(|k| {
                            html! { <option value={k.to_string()}>{k}</option> }
                        })}
                        </select>
                    </div>
                </div>
            </div>
        }
    }
}

fn checkbox<T>(label: String, checked: bool, cb: Callback<T>) -> Html
where Callback<T>: IntoPropValue<Option<Callback<ChangeData>>> {
    html! {
        <div>
            <input
                type="checkbox"
                checked={checked}
                onchange={cb}
            />
            {label}
        </div>
    }
}

fn dropdown<T>(pos: Position, label: String, cb: Callback<T>) -> Html
where Callback<T>: IntoPropValue<Option<Callback<ChangeData>>> {
    let normal = EnvironmentOption::Normal.to_string();
    let low = EnvironmentOption::Low.to_string();
    let high = EnvironmentOption::High.to_string();
    let ignore = EnvironmentOption::Ignore.to_string();
    let none = "".to_string();
    html! {
        <div>
            { if let Position::L = pos {&label} else {&none} }
            <select
                onchange={cb}
            >
                <option value={normal.clone()}>{normal}</option>
                <option value={low.clone()}>{low}</option>
                <option value={high.clone()}>{high}</option>
                <option value={ignore.clone()}>{ignore}</option>
            </select>
            { if let Position::R = pos {label} else {none} }
        </div>
    }
}
