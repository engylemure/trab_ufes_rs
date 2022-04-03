use std::{fmt, fs::File, io::Read};
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn distance(&self, point: &Point) -> f32 {
        let diff = (self.x - point.x).pow(2) + (self.y - point.y).pow(2);
        (diff as f32).sqrt()
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct City {
    pub name: String,
    pub location: Point,
    pub daily_rate: Option<f32>,
    pub travel_cost: Option<Vec<f32>>,
}

#[derive(Default, PartialEq, PartialOrd, Debug)]
pub struct DirectionalCities<'a> {
    pub north: Vec<&'a City>,
    pub south: Vec<&'a City>,
    pub west: Vec<&'a City>,
    pub east: Vec<&'a City>,
    pub center: Vec<&'a City>,
}

fn info_to<'a>(
    f: &mut fmt::Formatter<'_>,
    cities: &Vec<&'a City>,
    related_to: &str,
) -> fmt::Result {
    writeln!(f, "Cidade(s) mais ao {}", related_to)?;
    let cities_name: Vec<String> = cities.iter().map(|city| city.name.clone()).collect();
    writeln!(f, "{}", cities_name.join(", "))
}

impl<'a> fmt::Display for DirectionalCities<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        info_to(f, self.west.as_ref(), "Leste")?;
        info_to(f, self.east.as_ref(), "Oeste")?;
        info_to(f, self.south.as_ref(), "Sul")?;
        info_to(f, self.north.as_ref(), "Norte")?;
        info_to(f, self.center.as_ref(), "Centro")
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct State {
    pub cities: Vec<City>,
    distances_matrix: Option<Vec<Vec<f32>>>,
    costs: Option<Vec<Vec<f32>>>,
}

#[derive(Debug)]
pub struct SimplePath {
    pub cities_name: Vec<String>,
    pub cost: f32,
    pub distance: f32,
}

impl SimplePath {
    fn new() -> Self {
        Self {
            cities_name: Vec::new(),
            cost: 0.0,
            distance: 0.0,
        }
    }
}

impl Point {
    fn from_str(input: &str) -> Option<Self> {
        let mut input = input.trim().split(" ");
        Some(Point {
            x: input.next().unwrap().parse().ok().unwrap(),
            y: input.next().unwrap().parse().ok().unwrap(),
        })
    }
}

impl City {
    pub fn new(name: String, location: Point) -> Self {
        Self {
            name,
            location,
            daily_rate: None,
            travel_cost: None,
        }
    }
}

impl From<&str> for State {
    fn from(input: &str) -> Self {
        let mut cities = Vec::new();
        let mut lines = input.lines();
        while let (Some(name), Some(location)) = (lines.next(), lines.next()) {
            let location = Point::from_str(location).unwrap();
            cities.push(City::new(name.into(), location));
        }
        Self::new(cities)
    }
}

impl From<File> for State {
    fn from(mut file: File) -> Self {
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        Self::from(&content as &str)
    }
}

impl State {

    pub fn new(cities: Vec<City>) -> Self {
        Self {
            cities,
            costs: None,
            distances_matrix: None,
        }
    }

    pub fn read_costs(&mut self, input: &str) {
        let cities_len = self.cities.len();
        let mut lines = input.lines();
        for i in 0..cities_len {
            let city = self.cities.get_mut(i).unwrap();
            city.daily_rate = Some(lines.next().unwrap().parse().ok().unwrap());
        }

        for i in 0..cities_len {
            let travel_cost = lines
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|cost| cost.parse().unwrap())
                .collect();
            let city = self.cities.get_mut(i).unwrap();
            city.travel_cost = Some(travel_cost);
        }
    }

    pub fn read_costs_from_file(&mut self, mut file: File) {
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        self.read_costs(&content as &str)
    }

    pub fn distances_matrix(&mut self) -> &Vec<Vec<f32>> {
        self.distances_matrix.get_or_insert(
            self.cities
                .iter()
                .map(|city_a| {
                    self.cities
                        .iter()
                        .map(|city_b| city_a.location.distance(&city_b.location))
                        .collect()
                })
                .collect(),
        )
    }

    pub fn costs(&mut self) -> &Vec<Vec<f32>> {
        let distances_matrix = self.distances_matrix().clone();
        self.costs.get_or_insert(
                distances_matrix
                .into_iter()
                .enumerate()
                .map(|(i, distances)| {
                    let city_i = self.cities.get(i).unwrap();
                    distances
                        .into_iter()
                        .enumerate()
                        .map(|(j, dist)| {
                            let city_j = self.cities.get(j).unwrap();
                            dist * city_i.travel_cost.as_ref().unwrap()[j]
                                + city_j.daily_rate.unwrap()
                        })
                        .collect()
                })
                .collect(),
        )
    }

    pub fn calculate_directional_cities<'a>(&'a self) -> Option<DirectionalCities<'a>> {
        let mut cities_iter = self.cities.iter();
        let (mut north, mut east, mut south, mut west) = if let Some(city) = cities_iter.next() {
            (city, city, city, city)
        } else {
            return None;
        };
        for city in cities_iter {
            if city.location.x > west.location.x {
                west = city;
            }
            if city.location.x < east.location.x {
                east = city;
            }
            if city.location.y > north.location.y {
                north = city;
            }
            if city.location.y < south.location.y {
                south = city;
            }
        }
        let center = Point {
            x: (east.location.x + west.location.x) / 2,
            y: (north.location.y + south.location.y) / 2,
        };
        let mut lowest_d_center = self.cities.get(0)?.location.distance(&center);
        let distances_to_center: Vec<f32> = self
            .cities
            .iter()
            .map(|city| city.location.distance(&center))
            .collect();
        let mut d_cities = self
            .cities
            .iter()
            .enumerate()
            .map(|(i, city)| (city, distances_to_center[i]))
            .fold(
                DirectionalCities::default(),
                |mut d_cities, (city, d_center)| {
                    if city.location.x == west.location.x {
                        d_cities.west.push(city);
                    }
                    if city.location.x == east.location.x {
                        d_cities.east.push(city);
                    }
                    if city.location.y == north.location.y {
                        d_cities.north.push(city);
                    }
                    if city.location.y == south.location.y {
                        d_cities.south.push(city);
                    }
                    if lowest_d_center > d_center {
                        lowest_d_center = d_center;
                    }
                    d_cities
                },
            );
        for (i, d_center) in distances_to_center.into_iter().enumerate() {
            if d_center == lowest_d_center {
                d_cities.center.push(&self.cities[i])
            }
        }
        Some(d_cities)
    }

    pub fn simple_path(&self) -> SimplePath {
        self.cities
            .iter()
            .fold(SimplePath::new(), |mut path, city| path)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_name_and_location_from_str() {
        let input = r#"dores do rio preto
        3 6"#;
        assert_eq!(
            State::from(input).cities[0],
            City::new(String::from("dores do rio preto"), Point { x: 3, y: 6 })
        );
    }

    #[test]
    fn test_costs_from_str() {
        let input = r#"50
        0.00"#;
        let mut state = State::new(
            vec![City::new(
                String::from("dores do rio preto"),
                Point { x: 3, y: 6 },
            )]
        );
        state.read_costs(input);
        assert_eq!(
            state.cities[0],
            City {
                name: String::from("dores do rio preto"),
                location: Point { x: 3, y: 6 },
                daily_rate: Some(50.0),
                travel_cost: Some(vec![0.00])
            }
        );
        dbg!(state);
    }
}
