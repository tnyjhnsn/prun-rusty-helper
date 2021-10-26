use serde_derive::Deserialize;
use yew::html::ImplicitClone;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub struct Diagnostics {
    pub planets_with_env: usize,
    pub planets_with_res: usize,
    //pub planets_with_env_res: filter_hits.len()
    pub stars_with_planets_with_env_res: usize,
    pub filter_hits: Vec<(Planet, Resource)>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self {
            planets_with_env: 0,
            planets_with_res: 0,
            stars_with_planets_with_env_res: 0,
            filter_hits: Vec::new(),
        }
    }
    pub fn planets_with_env_res(&self) -> usize {
        self.filter_hits.len()
    }
}

#[allow(dead_code)]
#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Resource {
    pub planet: String,
    pub ticker: String,
    #[serde(rename(deserialize="type"))]
    pub typ: String,
    pub factor: f64,
    #[serde(skip_deserializing)]
    pub filtered: bool,
}

#[allow(dead_code)]
#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Planet {
    pub sys_id: String,
    pub name: String,
    pub nat_id: String,
    pub surface: bool,
    pub gravity: f64,
    pub temp: f64,
    pub pressure: f64,
    pub fertility: f64,
    #[serde(skip_deserializing)]
    pub filtered: bool,
}

impl ImplicitClone for Planet{}

const GRAVITY_MIN: f64 = 0.25;
const GRAVITY_MAX: f64 = 2.5;
const TEMP_MIN: f64 = -25.0;
const TEMP_MAX: f64 = 75.0;
const PRESSURE_MIN: f64 = 0.25;
const PRESSURE_MAX: f64 = 2.0;

impl Planet {
    pub fn apply_filters(&mut self, filters: &Filters) -> bool {
        self.filtered = false;
        self.filtered = if filters.env_filter {
            (match filters.surface {
                SurfaceOption::Rocky => self.surface,
                SurfaceOption::Gaseous => !self.surface,
                SurfaceOption::Both => true,
            })
            && self.check_extremes(&filters.gravity, self.gravity,
                        filters.inc_normal, GRAVITY_MIN, GRAVITY_MAX)
            && self.check_extremes(&filters.temp, self.temp,
                        filters.inc_normal, TEMP_MIN, TEMP_MAX)
            && self.check_extremes(&filters.pressure, self.pressure,
                        filters.inc_normal, PRESSURE_MIN, PRESSURE_MAX)

        } else {
            true
        };
        self.filtered
    }
    fn check_extremes(
        &self,
        environment: &EnvironmentOption,
        value: f64,
        inc_normal: bool,
        min: f64,
        max: f64,
        ) -> bool {
        match environment {
            EnvironmentOption::Low => value <
                if inc_normal { max } else { min },
            EnvironmentOption::High => value >
                if inc_normal { min } else { max },
            EnvironmentOption::Normal => min <= value && value <= max,
            EnvironmentOption::Ignore => true,
        }
    }
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Connection {
    pub connection: String,
}

#[allow(dead_code)]
#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Star {
    pub sys_id: String,
    pub name: String,
    pub nat_id: String,
    #[serde(rename(deserialize="type"))]
    pub typ: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub connections: Vec<Connection>,
    #[serde(skip_deserializing)]
    pub res_factor: f64,
}

impl ImplicitClone for Star{}

impl Star {
    pub fn new() -> Self {
        Self {
            sys_id: String::new(),
            name: String::new(),
            nat_id: String::new(),
            typ: String::new(),
            x: 0.0,
            y: 0.0,
            z: 0.0,
            connections: Vec::new(),
            res_factor: 0.0,
        }
    }
}

// TODO sort out what should be in Universe and what should be in PrUnApp
#[derive(PartialEq, Clone, Debug)]
pub struct Universe {
    pub stars: Vec<Star>,
    pub selected_star: Star,
    pub planets: Vec<Planet>,
    pub resources: Vec<Resource>,
    pub star_list: Vec<String>,
    pub res_list: Vec<String>,
    pub selected_res: Option<String>,
    pub res_max_factor: HashMap<String, f64>,
    pub diagnostics: Diagnostics,
}

impl ImplicitClone for Universe {}

impl Universe {
    pub fn new() -> Self {
        Self {
            stars: Vec::new(),
            selected_star: Star::new(),
            planets: Vec::new(),
            resources: Vec::new(),
            star_list: Vec::new(),
            res_list: Vec::new(),
            selected_res: None,
            res_max_factor: HashMap::new(),
            diagnostics: Diagnostics::new(),
        }
    }
    pub fn fix_star_y(&mut self) {
        for star in &mut self.stars {
            star.y *= -1.0;
        }
    }
    pub fn create_star_list(&mut self) {
        for star in &self.stars {
            self.star_list.push(star.nat_id.clone());
            if star.nat_id != star.name {
                self.star_list.push(star.name.clone());
            }
        };
        for planet in &self.planets {
            if planet.nat_id != planet.name {
                self.star_list.push(planet.name.clone());
            }
        };
    }
    pub fn create_resource_data(&mut self) {
        for resource in &self.resources {
            let ticker = &resource.ticker;
            let factor = match self.res_max_factor.get(ticker) {
                Some(&f) => f,
                None => 0.0,
            };

            self.res_max_factor.insert(ticker.to_string(), factor.max(resource.factor));
        }
        self.res_list = self.res_max_factor
            .keys()
            .cloned()
            .collect();
        self.res_list.sort();
        self.res_list.insert(0, "-None-".to_string());
    }
    // TODO Error checks to be added
    pub fn star_from_name(&self, name: String) -> Option<Star> {
        match name.chars().nth(2).unwrap() {
            '-' => {
                let text = &name[..6];
                let selected_star = self.stars.iter()
                    .find(|s| s.nat_id == text)
                    .unwrap()
                    .clone();
                Some(selected_star)
            }
            _ => {
                let planet = self.planets.iter().find(|p| {
                    p.name.to_ascii_uppercase() == name
                });
                if let Some(planet) = planet {
                    let selected_star = self.stars.iter()
                        .find(|s| s.sys_id == planet.sys_id)
                        .unwrap()
                        .clone();
                    Some(selected_star)
                } else {
                    let selected_star = self.stars.iter().find(|s| {
                        s.name.to_ascii_uppercase() == name
                    }).unwrap().clone();
                    Some(selected_star)
                }
            }
        }
    }
    pub fn apply_filters(&mut self, filters: &Filters) {

        self.diagnostics = Diagnostics::new();

        for planet in &mut self.planets {
            if planet.apply_filters(&filters) {
                self.diagnostics.planets_with_env += 1;
            }
        }

        self.resources
            .iter_mut()
            .for_each(|r| r.filtered = false);
        self.stars
            .iter_mut()
            .for_each(|s| s.res_factor = 0.0);


        match &self.selected_res {
            Some(res) => {
                for resource in &mut self.resources {
                    if resource.ticker.eq(res) {
                        resource.filtered = true;
                        self.diagnostics.planets_with_res += 1;
                        let planet = self.planets
                            .iter()
                            .find(|p| p.nat_id.eq(&resource.planet))
                            .unwrap();
                        if planet.filtered {
                            let star = self.stars
                                .iter_mut()
                                .find(|s| s.sys_id.eq(&planet.sys_id))
                                .unwrap();
                            star.res_factor = star.res_factor.max(resource.factor);
                            self.diagnostics.filter_hits.push((planet.clone(), resource.clone()));
                        }
                    }
                }
                self.diagnostics.filter_hits.sort_by(
                    |a, b| b.1.factor.partial_cmp(&a.1.factor).unwrap());
            }
            None => {
                for planet in self.planets.iter() {
                    if planet.filtered {
                        let star = self.stars
                            .iter_mut()
                            .find(|s| s.sys_id.eq(&planet.sys_id))
                            .unwrap();
                        star.res_factor = 1.0;
                    }
                }
            }
        }
        self.diagnostics.stars_with_planets_with_env_res = self.stars_with_planets_env_res();
    }
    pub fn stars_with_planets_env_res(&self) -> usize {
        self.stars
            .iter()
            .filter(|s| s.res_factor.gt(&0.0))
            .count()
    }
    pub fn planets_for_selected_star(&self) -> Vec<Planet> {
        self.planets
            .iter()
            .filter(|p| p.sys_id.eq(&self.selected_star.sys_id))
            .cloned()
            .collect()
    }
    pub fn resources_for_planet(&self, p: &Planet) -> Vec<Resource> {
        self.resources
            .iter()
            .filter(|r| r.planet.eq(&p.nat_id))
            .cloned()
            .collect()
    }
}

impl Default for Universe {
    fn default() -> Self {
        Self::new()
    }
}

pub enum Toggle {
    ShowCx,
    ShowRoutes,
    IncEnvFilter,
    IncNormal
}

pub type ScaleOptions = HashMap<String, (f64, bool)>;

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct MapFeatures {
    pub scale_options: ScaleOptions,
    pub selected_scale: f64,
    pub show_cx: bool,
    pub show_routes: bool,
}
impl ImplicitClone for MapFeatures{}

impl MapFeatures {
    pub fn new() -> Self {
        let mut scale_options = HashMap::new();
        scale_options.insert(String::from("Small"), (0.3, false));
        scale_options.insert(String::from("Medium"), (0.35, true));
        scale_options.insert(String::from("Large"), (0.4, false));
        Self {
            scale_options,
            selected_scale: 0.35,
            show_cx: true,
            show_routes: false,
        }
    }
    pub fn set_selected_scale(&mut self, selected: f64) {
        self.selected_scale = selected;
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum SurfaceOption {
    Rocky,
    Gaseous,
    Both,
}

impl SurfaceOption {
    pub fn to_string(&self) -> String {
        match self {
            Self::Rocky => String::from("Rocky"),
            Self::Gaseous => String::from("Gaseous"),
            Self::Both => String::from("Both"),
        }
    }
}

pub fn to_surface_option(s: &str) -> SurfaceOption {
    match s {
        "Rocky" => SurfaceOption::Rocky,
        "Gaseous" => SurfaceOption::Gaseous,
        _ => SurfaceOption::Both,
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Environment {
    Gravity,
    Temp,
    Pressure,
}

#[derive(PartialEq, Clone, Debug)]
pub enum EnvironmentOption {
    Normal,
    Low,
    High,
    Ignore,
}

impl EnvironmentOption {
    pub fn to_string(&self) -> String {
        match self {
            Self::Low => String::from("Low"),
            Self::Normal => String::from("Normal"),
            Self::High => String::from("High"),
            Self::Ignore => String::from("Ignore"),
        }
    }
}

pub fn to_env_option(s: &str) -> EnvironmentOption {
    match s {
        "Low" => EnvironmentOption::Low,
        "Normal" => EnvironmentOption::Normal,
        "High" => EnvironmentOption::High,
        _ => EnvironmentOption::Ignore,
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Filters {
    pub env_filter: bool,
    pub inc_normal: bool,
    pub surface: SurfaceOption,
    pub gravity: EnvironmentOption,
    pub temp: EnvironmentOption,
    pub pressure: EnvironmentOption,
    pub stars_filter_map: HashMap<String, f64>
}

impl Filters {
    pub fn new() -> Self {
        Self {
            env_filter: true,
            inc_normal: true,
            surface: SurfaceOption::Rocky,
            gravity: EnvironmentOption::Normal,
            temp: EnvironmentOption::Normal,
            pressure: EnvironmentOption::Normal,
            stars_filter_map: HashMap::new(),
        }
    }
}

impl ImplicitClone for Filters{}

#[allow(dead_code)]
pub enum Position {
    L,
    R,
}

