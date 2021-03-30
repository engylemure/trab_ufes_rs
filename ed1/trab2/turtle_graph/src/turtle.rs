use std::{
    collections::HashMap, convert::Infallible, f32::consts::TAU, fmt::Display, str::FromStr,
};

use crate::List;

#[derive(Debug)]
pub struct TurtleGraphConfig {
    angle: Option<u8>,
    order: Option<u8>,
    rotate: Option<i32>,
    axiom: Vec<TurtleSymbol>,
    rules: HashMap<TurtleSymbol, Vec<TurtleSymbol>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TurtleSymbol {
    F,
    G,
    Plus,
    Minus,
    PushStack,
    PopStack,
    CustomSymbol(char),
}

#[derive(Debug)]
pub struct TurtleSyntax {
    list: List<TurtleSymbol>,
    angle: f32,
    order: u8,
    rotate: f32,
}

impl From<char> for TurtleSymbol {
    fn from(c: char) -> Self {
        match c {
            'F' => Self::F,
            'G' => Self::G,
            '+' => Self::Plus,
            '-' => Self::Minus,
            '[' => Self::PushStack,
            ']' => Self::PopStack,
            c => Self::CustomSymbol(c),
        }
    }
}

impl Display for TurtleSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            TurtleSymbol::F => 'F',
            TurtleSymbol::G => 'G',
            TurtleSymbol::Plus => '+',
            TurtleSymbol::Minus => '-',
            TurtleSymbol::PushStack => '[',
            TurtleSymbol::PopStack => ']',
            TurtleSymbol::CustomSymbol(c) => *c,
        };
        write!(f, "{}", c)
    }
}

impl FromStr for TurtleGraphConfig {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (angle, order, rotate, axiom, rules) =
            s.lines()
                .fold((None, None, None, None, HashMap::new()), |mut acc, line| {
                    let line = line.trim();
                    let line_end = line.find(';').unwrap_or(line.len());
                    let word = &line[0..line_end];
                    if word.starts_with("angle") {
                        acc.0 = word
                            .strip_prefix("angle")
                            .map(|w| str::parse(w.trim()))
                            .map(Result::ok)
                            .flatten()
                    } else if word.starts_with("order") {
                        acc.1 = word
                            .strip_prefix("order")
                            .map(|w| str::parse(w.trim()))
                            .map(Result::ok)
                            .flatten()
                    } else if word.starts_with("rotate") {
                        acc.2 = word
                            .strip_prefix("rotate")
                            .map(|w| str::parse(w.trim()))
                            .map(Result::ok)
                            .flatten()
                    } else if word.starts_with("axiom") {
                        acc.3 = word
                            .strip_prefix("axiom")
                            .map(|w| w.trim().chars().map(TurtleSymbol::from).collect())
                    } else {
                        let rule = word.replace(" ", "");
                        let mut chars = rule.chars();
                        if let (Some(symbol), Some('=')) =
                            (chars.next().map(TurtleSymbol::from), chars.next())
                        {
                            acc.4
                                .insert(symbol, chars.map(TurtleSymbol::from).collect());
                        }
                    }
                    acc
                });
        Ok(Self {
            angle,
            order,
            rotate,
            axiom: axiom.unwrap_or_default(),
            rules,
        })
    }
}

impl TurtleGraphConfig {
    pub fn generate_syntax(&self) -> TurtleSyntax {
        let mut syntax = TurtleSyntax {
            list: self.axiom.clone().into_iter().collect(),
            angle: self.angle.unwrap_or(0) as f32,
            order: self.order.unwrap_or(0),
            rotate: self.rotate.unwrap_or(0) as f32,
        };
        if let Some(order) = self.order {
            for i in 0..order {
                for (symbol, value) in &self.rules {
                    syntax.apply_axiom(symbol, value);
                }
            }
        }
        syntax
    }
}

#[derive(Debug)]
struct TurtleSyntaxState {
    x: f32,
    y: f32,
    z: f32,
    angle: f32,
    color: Colors,
}

impl TurtleSyntaxState {
    fn new(x: f32, y: f32, z: f32, angle: f32, color: Colors) -> Self {
        Self {
            x,
            y,
            z,
            angle,
            color,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Colors {
    Red,
    Green,
    DarkBlue,
    Black,
    Brown,
    DarkGreen,
    White,
}

impl Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_value = match self {
            Colors::Red => "1 0 0",
            Colors::Green => "0 1 0",
            Colors::DarkBlue => "0 0 1",
            Colors::Black => "0 0 0",
            Colors::Brown => "0.7 0.3 0",
            Colors::DarkGreen => "0.0 0.5 0.0",
            Colors::White => "1 1 1",
        };
        write!(f, "{} setrgbcolor", color_value)
    }
}

impl TurtleSyntax {
    pub fn apply_axiom(&mut self, symbol: &TurtleSymbol, value: &Vec<TurtleSymbol>) {
        for node in self.list.iter_mut() {
            if &node.data == symbol {
                node.replace_with_list(value.clone().into_iter().collect());
            }
        }
    }

    pub fn convert(&self) -> String {
        let mut value: String = include_str!("preamble/preamble_header.txt").into();
        let base_order = std::env::var("BASE_ORDER")
            .unwrap_or("1".into())
            .parse()
            .unwrap_or(1);
        value.push_str(&format!(
            "/angle {:.2} def\n/order {} def\n/rotateimage {:.2} def\n",
            self.angle,
            self.order / base_order,
            self.rotate
        ));
        value.push_str(include_str!("preamble/preamble_content.txt"));
        value.push_str(&self.content());
        value.push_str("stroke\n\ngrestore\n\nshowpage\nquit\n");
        value
    }
    fn content(&self) -> String {
        let mut value = String::new();
        let mut x = 0f32;
        let mut y = 0f32;
        let mut z = 100f32;
        let mut color = Colors::Black;
        let base_angle = TAU / self.angle;
        let mut angle = 0f32;
        let mut history_stack: List<TurtleSyntaxState> = List::new();
        let mut iter = self.list.iter();
        while let Some(node) = iter.next() {
            match node.data {
                TurtleSymbol::F => {
                    value.push_str(&format!("{}\n", color));
                    value.push_str(&format!("n {:.2} {:.2} ", x, y));
                    x += angle.cos() * z;
                    y += angle.sin() * z;
                    value.push_str(&format!("m {:.2} {:.2} l s\n", x, y));
                }
                TurtleSymbol::G => {
                    x += angle.cos() * z;
                    y += angle.sin() * z;
                }
                TurtleSymbol::Plus => {
                    angle += base_angle;
                }
                TurtleSymbol::Minus => {
                    angle -= base_angle;
                }
                TurtleSymbol::PushStack => {
                    history_stack.push(TurtleSyntaxState::new(x, y, z, angle, color));
                }
                TurtleSymbol::PopStack => {
                    if let Some(state) = history_stack.pop_tail() {
                        x = state.x;
                        y = state.y;
                        z = state.z;
                        angle = state.angle;
                        color = state.color;
                    }
                }
                TurtleSymbol::CustomSymbol(c) if c == 'C' => {
                    if let Some(node) = iter.next() {
                        if let TurtleSymbol::CustomSymbol(color_value) = node.data {
                            match color_value {
                                '0' => color = Colors::Black,
                                '1' => color = Colors::Red,
                                '2' => color = Colors::DarkBlue,
                                '3' => color = Colors::Green,
                                '4' => color = Colors::Brown,
                                '5' => color = Colors::DarkGreen,
                                '6' => color = Colors::White,
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        value
    }

    pub fn string(&self) -> String {
        self.list
            .iter()
            .map(|node| format!("{}", node.data))
            .collect::<Vec<String>>()
            .join("")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const helloworld: &str = r#"angle 8 ; means 360/8
order 2
axiom ++F
F = F+F
    "#;
    #[test]
    fn basic_config() {
        let preamble = TurtleGraphConfig::from_str(helloworld).unwrap();
        assert_eq!(preamble.angle, Some(8));
        assert_eq!(preamble.order, Some(2));
        assert_eq!(preamble.rotate, None);
        assert_eq!(
            preamble.axiom,
            vec![TurtleSymbol::Plus, TurtleSymbol::Plus, TurtleSymbol::F]
        );
        dbg!(&preamble);
        dbg!(preamble.generate_syntax());
    }
}
