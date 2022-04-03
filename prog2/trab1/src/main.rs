use std::{convert::TryInto, io, cmp::min};

mod lib;
use lib::*;
// use rand::Rng;

fn read_answers() -> String {
    let mut answers = String::new();
    for _ in 0..20 {
        io::stdin()
            .read_line(&mut answers)
            .expect("Failed to read line!");
    }
    answers
}

fn read_candidates() -> Vec<Candidate> {
    let mut candidates = Vec::new();
    for _ in 0..100 {
        let mut id = String::new();
        io::stdin()
            .read_line(&mut id)
            .expect("Failed to read line!");
        let id: i32 = id.trim().parse().expect("Invalid Candidate ID");
        if id == -1 {
            break;
        } else {
            let id: u32 = id.try_into().unwrap();
            let mut answers = Vec::new();
            for _ in 0..20 {
                answers.push(read_candidate_answer());
            }
            candidates.push(Candidate::new(id, answers))
        }
    }
    candidates
}

fn read_candidate_answer() -> CandidateAnswer {
    let mut answer = String::new();
    for _ in 0..4 {
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line!");
    }
    CandidateAnswer::new(answer)
}

fn normal_program()  {
    let answers = read_answers();
    let candidates = read_candidates();
    let processor = ExamProcessor::new(answers);
    let ExamResult { mut approved, eliminated} = processor.process(&candidates);
    for candidate in eliminated {
        println!("Candidato Eliminado\nMATRICULA={}", candidate.id)
    }
    let approved: Vec<CandidateResultInfo> = approved.drain(..(min(2, approved.len()))).collect();
    for (index, candidate) in approved.iter().enumerate() {
        println!("Habilitado {}\nMATRICULA={}\nPONTUACAO={:.2}", index + 1, candidate.id, candidate.total_grade)
    }
}

fn main() {
    normal_program();
    // let options = ["a", "b", "c", "d"];
    // for _ in 0..20 {
    //     let option_idx = rand::thread_rng().gen_range(0, options.len());
    //     match options[option_idx] {
    //         "a" => println!("1\n0\n0\n0"),
    //         "b" => println!("0\n1\n0\n0"),
    //         "c" => println!("0\n0\n1\n0"),
    //         "d" => println!("0\n0\n0\n1"),
    //         _ => ()
    //     }
    // }
}
