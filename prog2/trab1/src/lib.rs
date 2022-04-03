use std::cmp::Ordering;

#[derive(Debug)]
pub struct ExamProcessor {
    pub answers: ExamAnswers,
}

impl ExamProcessor {
    pub fn new(answers: String) -> Self {
        Self {
            answers: ExamAnswers::new(answers),
        }
    }

    pub fn process(&self, candidates: &Vec<Candidate>) -> ExamResult {
        let mut eliminated = Vec::new();
        let mut approved = Vec::new();
        for candidate in candidates {
            match self.process_candidate(candidate) {
                CandidateResult::Approved(candidate) => {
                    approved.push(candidate);
                }
                CandidateResult::Eliminated(candidate) => eliminated.push(candidate),
            }
        }
        approved.sort();
        ExamResult {
            eliminated,
            approved,
        }
    }

    fn process_candidate(&self, candidate: &Candidate) -> CandidateResult {
        let mut grade: (f32, f32, f32, f32) = (0.0, 0.0, 0.0, 0.0);
        for (index, answer) in candidate.answers.iter().enumerate() {
            let expected_answer = self.answers.values.get(index).unwrap();
            let is_correct = expected_answer == answer;
            match index {
                0..=4 => {
                    if is_correct {
                        grade.0 += 2.5;
                    }
                }
                5..=7 => {
                    if is_correct {
                        grade.1 += 1.5;
                    }
                }
                8..=9 => {
                    if is_correct {
                        grade.2 += 1.0;
                    }
                }
                10..=19 => {
                    if is_correct {
                        grade.3 += 3.5;
                    }
                }
                _ => panic!(
                    "Candidate with id: {}, has more than 20 answers",
                    candidate.id
                ),
            }
        }
        if grade.0 < 5.0 || grade.1 < 1.5 || grade.2 < 1.0 || grade.3 < 14.0 {
            CandidateResult::Eliminated(CandidateResultInfo::new(candidate.id, grade))
        } else {
            CandidateResult::Approved(CandidateResultInfo::new(candidate.id, grade))
        }
    }
}

#[derive(Debug)]
pub struct ExamResult {
    pub eliminated: Vec<CandidateResultInfo>,
    pub approved: Vec<CandidateResultInfo>,
}
#[derive(Debug)]
pub enum CandidateResult {
    Approved(CandidateResultInfo),
    Eliminated(CandidateResultInfo),
}

#[derive(Debug)]
pub struct CandidateResultInfo {
    pub id: u32,
    pub total_grade: f32,
    pub grades: (f32, f32, f32, f32),
}

impl CandidateResultInfo {
    pub fn new(id: u32, grades: (f32, f32, f32, f32)) -> Self {
        Self {
            id,
            total_grade: grades.0 + grades.1 + grades.2 + grades.3,
            grades,
        }
    }
}

impl Ord for CandidateResultInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        let total_grade_cmp = other
            .total_grade
            .partial_cmp(&self.total_grade)
            .unwrap_or(Ordering::Equal);
        if total_grade_cmp == Ordering::Equal {
            let fourth_grade_cmp = other
                .grades
                .3
                .partial_cmp(&self.grades.3)
                .unwrap_or(Ordering::Equal);
            if fourth_grade_cmp == Ordering::Equal {
                let first_grade_cmp = other
                    .grades
                    .0
                    .partial_cmp(&self.grades.0)
                    .unwrap_or(Ordering::Equal);
                if first_grade_cmp == Ordering::Equal {
                    let second_grade_cmp = other
                        .grades
                        .1
                        .partial_cmp(&self.grades.1)
                        .unwrap_or(Ordering::Equal);
                    if second_grade_cmp == Ordering::Equal {
                        other.grades
                            .2
                            .partial_cmp(&self.grades.2)
                            .unwrap_or(Ordering::Equal)
                    } else {
                        second_grade_cmp
                    }
                } else {
                    first_grade_cmp
                }
            } else {
                fourth_grade_cmp
            }
        } else {
            total_grade_cmp
        }
    }
}

impl PartialOrd for CandidateResultInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CandidateResultInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for CandidateResultInfo {}

#[derive(Debug, PartialEq)]
pub enum Answer {
    A,
    B,
    C,
    D,
}
#[derive(Debug)]
pub struct ExamAnswers {
    pub values: Vec<Answer>,
}

impl ExamAnswers {
    pub fn new(answers: String) -> Self {
        let answers = {
            let mut data = Vec::new();
            for line in answers.lines() {
                data.push(Answer::from(line))
            }
            data
        };
        Self { values: answers }
    }
}

impl From<char> for Answer {
    fn from(c: char) -> Self {
        match c {
            'a' => Answer::A,
            'b' => Answer::B,
            'c' => Answer::C,
            'd' => Answer::D,
            _ => panic!("Unknown Answer '{}'", c),
        }
    }
}

impl From<String> for Answer {
    fn from(s: String) -> Self {
        match s.trim() {
            "a" => Answer::A,
            "b" => Answer::B,
            "c" => Answer::C,
            "d" => Answer::D,
            _ => panic!("Unknown Answer \"{}\"", s),
        }
    }
}

impl From<&str> for Answer {
    fn from(s: &str) -> Self {
        match s.trim() {
            "a" => Answer::A,
            "b" => Answer::B,
            "c" => Answer::C,
            "d" => Answer::D,
            _ => panic!("Unknown Answer '{}'", s),
        }
    }
}

impl From<CandidateAnswer> for Answer {
    fn from(s: CandidateAnswer) -> Self {
        match s.0.trim().replace(" ", "").as_str() {
            "1\n0\n0\n0" => Answer::A,
            "0\n1\n0\n0" => Answer::B,
            "0\n0\n1\n0" => Answer::C,
            "0\n0\n0\n1" => Answer::D,
            _ => panic!("Unknown Answer \"{}\"", s.0),
        }
    }
}

#[derive(Debug)]
pub struct Candidate {
    pub id: u32,
    pub answers: Vec<Answer>,
}

impl Candidate {
    pub fn new(id: u32, answers: Vec<CandidateAnswer>) -> Self {
        Self {
            id,
            answers: answers.into_iter().map(Answer::from).collect(),
        }
    }
}
#[derive(Debug)]
pub struct CandidateAnswer(String);

impl CandidateAnswer {
    pub fn new(answer: String) -> Self {
        Self(answer)
    }
}
