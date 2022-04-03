mod lib;
use lib::*;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;

fn handle_costs(state: &mut State, file_path: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .expect(&format!("O arquivo '{}' nao pode ser aberto!", file_path));
    for cost in state.costs() {
        let line: String = cost.iter().map(|c| format!("{:.6} ", c)).collect();
        writeln!(&mut file, "{}", line)
            .expect(&format!("Erro ao escrever no arquivo '{}'!", file_path));
    }
}

fn handle_distances(state: &mut State, file_path: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .expect(&format!("O arquivo '{}' nao pode ser aberto!", file_path));
    for distances in state.distances_matrix() {
        let line: String = distances.iter().map(|c| format!("{:.2} ", c)).collect();
        writeln!(&mut file, "{}", line)
            .expect(&format!("Erro ao escrever no arquivo '{}'!", file_path));
    }
}

fn handle_directional_cities(state: &State) {
    if let Some(d_cities) = state.calculate_directional_cities() {
        println!("{}", d_cities);
    }
}
enum UserOptions {
    Invalid,
    Position,
    Quit,
}

impl From<u32> for UserOptions {
    fn from(option: u32) -> Self {
        match option {
            1 => Self::Position,
            7 => Self::Quit,
            _ => Self::Invalid,
        }
    }
}

fn main() {
    let name_and_location_file = File::open("input/nome-coord.txt").unwrap();
    let cost_file = File::open("input/diaria-custo.txt").unwrap();
    let mut state = State::from(name_and_location_file);
    state.read_costs_from_file(cost_file);
    handle_costs(&mut state, "custo.txt");
    handle_distances(&mut state, "distancia.txt");
    let mut user_input = String::new();
    loop {
        println!("Menu de opcoes:");
        println!("1 - Funcao Posicao");
        println!("2 - Funcao Caminho");
        println!("3 - Funcao Lei de Formacao");
        println!("7 - Sair");
        print!("Opcao: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Linha nao pode ser lida");
        let user_option = user_input.trim().parse::<u32>();
        match user_option
            .ok()
            .map(UserOptions::from)
            .unwrap_or(UserOptions::Invalid)
        {
            UserOptions::Invalid => println!("Digite uma opcao valida!"),
            UserOptions::Position => handle_directional_cities(&state),
            UserOptions::Quit => break,
        }
        user_input.clear();
    }
}
