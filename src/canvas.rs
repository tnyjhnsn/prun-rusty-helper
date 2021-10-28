use yew::prelude::*;
use std::f64;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

use crate::models::{MapFeatures, Star, Universe};

#[derive(PartialEq, Clone, Debug)]
pub struct Map {
    pub width: f64,
    pub height: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub show_cx: bool,
    pub show_path: bool,
}

impl Map {
    pub fn new() -> Self {
        Self {
            width: 1700.0,
            height: 1700.0,
            offset_x: 1400.0,
            offset_y: 1700.0,
            show_cx: true,
            show_path: false,
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub map_features: MapFeatures,
    pub universe: Universe,
    pub env_filter: bool,
    pub selected_star_signal: Callback<Star>,
}

#[allow(dead_code)]
pub struct Canvas {
    canvas: NodeRef,
    map: Map,
    current_star: Star,
    //last_selected_star: Star,
    cx: Vec<&'static str>,
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    MouseDown(MouseEvent),
    MouseMove(MouseEvent)
}

struct StarColour;
#[allow(dead_code)]
impl StarColour {
    pub const PINPOINT: &'static str = "rgba(69,90,100,1.0)";
    pub const ENV_ONLY: &'static str = "rgba(76,175,80,1.0)";
    pub const PATH: &'static str = "rgba(255,255,255,0.6)";
    pub const LAST_STAR_HALO: &'static str = "rgba(233,30,99,1.0)";
    pub const SELECTED_STAR_HALO: &'static str = "rgba(255,235,59,1.0)";
    pub const CURRENT: &'static str = "#2196f3";
}

struct StarSize;
#[allow(dead_code)]
impl StarSize {
    pub const PINPOINT: f64 = 5.0;
    pub const SMALL: f64 = 8.0;
    pub const LARGE: f64 = 10.0;
    pub const CX: f64 = 17.0;
}

#[allow(dead_code)]
enum Fill {
    Filled,
    Halo,
    Cx,
}

impl Canvas {

    fn get_ctx(&self) -> CanvasRenderingContext2d {
        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
        canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap()
    }

    fn draw_star_circle(
        &self, ctx: &CanvasRenderingContext2d,
        x: f64, y: f64, size: f64, col: &str, fill: Fill) {
        match fill {
            Fill::Filled => {
                ctx.begin_path();
                ctx.arc(x, y, size, 0.0, 2.0 * f64::consts::PI).unwrap();
                ctx.set_fill_style(&JsValue::from_str(&col));
                ctx.fill();
            }
            Fill::Halo => {
                ctx.set_line_width(size);
                ctx.set_stroke_style(&JsValue::from_str(&col));
                for i in 0..4 {
                    let j = i as f64;
                    ctx.begin_path();
                    ctx.arc(x, y, size * 2.0,
                        (0.1+(0.5*j)) * f64::consts::PI,
                        (0.4+(0.5*j)) * f64::consts::PI)
                        .unwrap();
                    ctx.stroke();
                }
            }
            Fill::Cx => {
                ctx.set_line_width(2.0);
                ctx.set_stroke_style(&JsValue::from_str(&col));
                ctx.begin_path();
                ctx.arc(x, y, size, 0.0, 2.0 * f64::consts::PI).unwrap();
                ctx.set_fill_style(&JsValue::from_str(&col));
                ctx.stroke();
            }
        }
    }

    fn draw(&self) {
        let ctx = self.get_ctx();
        let width = self.map.width;
        let height = self.map.height;
        let scale = self.props.map_features.selected_scale;
        let show_cx = self.props.map_features.show_cx;
        let offset_x = self.map.offset_x;
        let offset_y = self.map.offset_y;

        ctx.restore();
        ctx.save();
        ctx.scale(scale, scale).unwrap();
        ctx.translate(offset_x, offset_y).unwrap();
        ctx.clear_rect(0.0-offset_x, 0.0-offset_y, width/scale, height/scale);

        for star in &self.props.universe.stars {
            let x = star.x.round();
            let y = star.y.round();

            let is_current = self.current_star.sys_id == star.sys_id;
            let is_selected = match &self.props.universe.selected_star {
                Some(s) => s.sys_id == star.sys_id,
                None => false,
            };
            //let is_last_selected = self.last_selected_star.sys_id == star.sys_id;

            if show_cx && self.cx.iter().any(|&i| i == star.nat_id) {
                 self.draw_star_circle(&ctx, x, y, StarSize::CX,
                    StarColour::LAST_STAR_HALO, Fill::Cx);
            }

            let res_max_factor = &self.props.universe.res_max_factor;
            let selected_res = &self.props.universe.selected_res;

            if star.res_factor.gt(&0.0) {
                match selected_res {
                    Some(res) => {
                        let max_factor =
                            res_max_factor.get(res).unwrap();
                        let conc = star.res_factor/max_factor;
                        let colour = match conc {
                            c if c >= 0.66 => "#4caf50",
                            c if c >= 0.33 => "#ff9800",
                            _=> "#f44336",
                        };
                        self.draw_star_circle(&ctx, x, y, StarSize::LARGE, colour, Fill::Filled)
                    }
                    None => {
                        match self.props.env_filter {
                            true => self.draw_star_circle(&ctx, x, y,
                                    StarSize::LARGE, StarColour::ENV_ONLY, Fill::Filled),
                            false => self.draw_star_circle(&ctx, x, y,
                                   StarSize::PINPOINT, StarColour::PINPOINT, Fill::Filled),
                        }
                    }
                }

            } else {
                self.draw_star_circle(&ctx, x, y, StarSize::PINPOINT, StarColour::PINPOINT, Fill::Filled)
            }

            if is_selected {
                self.draw_star_circle(&ctx, x, y, StarSize::LARGE,
                    StarColour::SELECTED_STAR_HALO, Fill::Halo);

                let s = format!("{} (Type {})", star.name, star.typ);

                ctx.set_font("40px 'Open Sans'");
                ctx.set_fill_style(&JsValue::from_str(&"rgba(255,255,255,1.0"));
                ctx.fill_text(&s, x + 20.0, y - 15.0).unwrap();
                let tm = ctx.measure_text(&s).unwrap();
                ctx.set_fill_style(&JsValue::from_str(&"rgba(158,158,158,0.15"));
                ctx.fill_rect(x + 5.0, y - 70.0, tm.width() + 30.0, 80.0);
                ctx.set_line_width(2.0);
                ctx.set_stroke_style(&JsValue::from_str(&"rgba(255,255,255,0.3"));
                ctx.stroke_rect(x + 5.0, y - 70.0, tm.width() + 30.0, 80.0);
            }

            if is_current {
                self.draw_star_circle(&ctx, x, y, StarSize::SMALL,
                    StarColour::CURRENT, Fill::Filled);
                ctx.set_font("40px 'Open Sans'");
                ctx.set_fill_style(&JsValue::from_str(&"rgba(255,255,255,1.0"));
                ctx.fill_text(&star.name, x + 20.0, y - 15.0).unwrap();
            }
        }
    }
}

impl Component for Canvas {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            canvas: NodeRef::default(),
            map: Map::new(),
            current_star: Star::new(),
            //last_selected_star: Star::new(),
            cx: vec!["OT-580", "UV-351", "VH-331", "ZV-307"],
            link,
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            self.draw();
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MouseDown(_e) => {
                let star = &self.current_star;
                //self.last_selected_star = self.props.universe.selected_star.clone();
                self.props.selected_star_signal.emit(star.clone());
                self.draw();
            }
            Msg::MouseMove(e) => {
                let scale = self.props.map_features.selected_scale;
                let offset_x = self.map.offset_x;
                let offset_y = self.map.offset_y;
                let mut best_fit = Star::new();
                let mut distance = 0.0;
                let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
                let bx = canvas.get_bounding_client_rect();
                let x1 = (e.client_x() as f64 - bx.left()) as f64 /scale - offset_x;
                let y1 = (e.client_y() as f64 - bx.top()) as f64 /scale - offset_y;

                for star in &self.props.universe.stars {
                    let d = (star.x - x1).powf(2.0) + (star.y - y1).powf(2.0);
                    if best_fit.sys_id == "" {
                        best_fit = star.clone();
                        distance = d;
                    } else if d < distance {
                        best_fit = star.clone();
                        distance = d;
                    };

                }
                self.current_star = best_fit;
                self.draw();
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="canvas-container">
                <canvas
                    ref=self.canvas.clone()
                    width=1700
                    height=1700
                    onmousedown=self.link.callback(Msg::MouseDown)
                    onmousemove=self.link.callback(Msg::MouseMove)
                />
            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}
