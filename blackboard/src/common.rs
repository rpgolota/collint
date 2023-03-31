use bimap::BiHashMap;
use rand::{seq::SliceRandom, Rng};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Problem {
    first: String,
    second: String,
    result: String,
    letters: HashSet<char>,
}

impl Problem {
    pub fn new(first: &str, second: &str, result: &str) -> Self {
        let first = first.to_lowercase();
        let second = second.to_ascii_lowercase();
        let result = result.to_lowercase();

        let mut letters: HashSet<char> = first.chars().collect();
        letters.extend(second.chars());
        letters.extend(result.chars());

        let mut hs: BiHashMap<char, u32> = BiHashMap::new();
        hs.extend(
            letters
                .iter()
                .copied()
                .zip((0..=letters.len()).map(|x| x as u32)),
        );

        Problem {
            first,
            second,
            result,
            letters,
        }
    }
}

impl Default for Problem {
    fn default() -> Self {
        Problem::new("DONALD", "GERALD", "ROBERT")
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct LetterAssignment {
    letter: char,
    value: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum Hint {
    One(LetterAssignment),
    Two(LetterAssignment, LetterAssignment),
    Three(LetterAssignment, LetterAssignment, LetterAssignment),
}

impl PartialEq for Hint {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Hint::One(a), Hint::One(a1)) => a == a1,
            (Hint::Two(a, b), Hint::Two(c, d)) => (a == c && b == d) || (a == d && b == c),
            (Hint::Three(a, b, c), Hint::Three(a1, b1, c1)) => {
                let h: HashSet<LetterAssignment> = [*a, *b, *c].iter().copied().collect();
                let h1: HashSet<LetterAssignment> = [*a1, *b1, *c1].iter().copied().collect();
                h == h1
            }
            _ => false,
        }
    }
}

impl Hint {
    fn is_correct(&self) -> bool {
        let p = Problem::default();
        let mut a: Agent = Agent::new(&p, false);
        a.assign_random();
        a.assimilate_hint(self);
        if *a.assignment.get_by_left(&'d').unwrap() == 5
            && *a.assignment.get_by_left(&'g').unwrap() == 1
            && *a.assignment.get_by_left(&'r').unwrap() == 7
        {
            return true;
        }
        if *a.assignment.get_by_left(&'o').unwrap() == 2
            && *a.assignment.get_by_left(&'e').unwrap() == 9
        {
            return true;
        }
        if *a.assignment.get_by_left(&'n').unwrap() == 6
            && *a.assignment.get_by_left(&'r').unwrap() == 7
            && *a.assignment.get_by_left(&'b').unwrap() == 3
        {
            return true;
        }
        if *a.assignment.get_by_left(&'a').unwrap() == 4
            && *a.assignment.get_by_left(&'e').unwrap() == 9
        {
            return true;
        }
        if *a.assignment.get_by_left(&'l').unwrap() == 8
            && *a.assignment.get_by_left(&'r').unwrap() == 7
        {
            return true;
        }
        if *a.assignment.get_by_left(&'d').unwrap() == 5
            && *a.assignment.get_by_left(&'t').unwrap() == 0
        {
            return true;
        }

        false
    }
}

#[derive(Debug, Clone)]
pub struct Agent<'a> {
    problem: &'a Problem,
    hints: Vec<Hint>,
    assignment: BiHashMap<char, u32>,
    pub cost: u32,
    pub correct_hints: u32,
    pub total_hints: u32,
    pub compute_phi: bool,
}

impl<'a> Agent<'a> {
    pub fn new(problem: &'a Problem, compute_phi: bool) -> Self {
        let mut assignment: BiHashMap<char, u32> = BiHashMap::new();
        assignment.extend(
            problem
                .letters
                .iter()
                .copied()
                .zip((0..=problem.letters.len()).map(|x| x as u32)),
        );
        Agent {
            problem,
            hints: Vec::new(),
            assignment,
            cost: 0,
            correct_hints: 0,
            total_hints: 0,
            compute_phi,
        }
    }
}

impl<'a> Agent<'a> {
    pub fn assign_random(&mut self) {
        let mut digits: Vec<u32> = (0..=9).collect();
        for (l, _) in self.assignment.clone().iter() {
            let to_remove = rand::thread_rng().gen_range(0..digits.len());
            let random = digits.remove(to_remove);
            self.assignment.insert(*l, random);
        }
    }
    pub fn find_hints(&mut self) {
        self.hints.clear();

        for i in 0..self.problem.first.len() {
            // Get character at index i
            let a1 = self.problem.first.chars().nth(i).unwrap();
            let a2 = self.problem.second.chars().nth(i).unwrap();
            let r = self.problem.result.chars().nth(i).unwrap();

            // get digit assigned to that character
            let a1_d = self.assignment.get_by_left(&a1).unwrap();
            let a2_d = self.assignment.get_by_left(&a2).unwrap();
            let r_d = self.assignment.get_by_left(&r).unwrap();

            let cond1 = (a1_d + a2_d) % 10 == *r_d;
            let cond2 = (a1_d + a2_d + 1) % 10 == *r_d;

            if cond1 || cond2 {
                let hint: HashSet<LetterAssignment> = [
                    LetterAssignment {
                        letter: a1,
                        value: *a1_d,
                    },
                    LetterAssignment {
                        letter: a2,
                        value: *a2_d,
                    },
                    LetterAssignment {
                        letter: r,
                        value: *r_d,
                    },
                ]
                .iter()
                .copied()
                .collect();
                let hint: Vec<LetterAssignment> = hint.iter().copied().collect();
                let hint = match hint.len() {
                    1 => Hint::One(*hint.get(0).unwrap()),
                    2 => Hint::Two(*hint.get(0).unwrap(), *hint.get(1).unwrap()),
                    3 => Hint::Three(
                        *hint.get(0).unwrap(),
                        *hint.get(1).unwrap(),
                        *hint.get(2).unwrap(),
                    ),
                    _ => unreachable!(),
                };
                self.hints.push(hint);
                // println!("fount hint: {:?}", hint);
            }
        }
    }
    pub fn pick_and_replace(&mut self, blackboard: &mut Vec<Hint>) {
        let novel: Vec<&Hint> = self
            .hints
            .iter()
            .filter(|hint| !blackboard.contains(*hint))
            .collect();
        if novel.is_empty() {
            return;
        }
        let selected = *novel.choose(&mut rand::thread_rng()).unwrap();

        if blackboard.len() == blackboard.capacity() {
            let different: Vec<usize> = blackboard
                .iter()
                .enumerate()
                .filter(|(_, hint)| !self.hints.contains(hint))
                .map(|(i, _)| i)
                .collect();
            if different.is_empty() {
                return;
            }
            let to_replace = different.choose(&mut rand::thread_rng()).unwrap();
            blackboard.remove(*to_replace);
            blackboard.push(*selected);
        } else {
            blackboard.push(*selected);
        }
    }
    pub fn make_move(&mut self, blackboard: &mut Vec<Hint>) {
        if !blackboard.is_empty() {
            let random_hint = blackboard.choose(&mut rand::thread_rng()).unwrap();
            if self.compute_phi {
                self.total_hints += 1;
                if random_hint.is_correct() {
                    self.correct_hints += 1;
                }
            }
            if !self.hints.contains(random_hint) {
                self.assimilate_hint(random_hint);
                return;
            }
        }
        self.elementary_move();
    }
    pub fn is_solved(&self) -> bool {
        let base: u32 = 10;

        let mut first_n: u32 = 0;
        for (i, l) in self.problem.first.chars().rev().enumerate() {
            first_n += base.pow(i as u32) * self.assignment.get_by_left(&l).unwrap();
        }

        let mut second_n: u32 = 0;
        for (i, l) in self.problem.second.chars().rev().enumerate() {
            second_n += base.pow(i as u32) * self.assignment.get_by_left(&l).unwrap();
        }

        let mut result_n: u32 = 0;
        for (i, l) in self.problem.result.chars().rev().enumerate() {
            result_n += base.pow(i as u32) * self.assignment.get_by_left(&l).unwrap();
        }

        first_n + second_n == result_n
    }
    pub fn compute_cost(&mut self) {
        let base: u32 = 10;

        if *self
            .assignment
            .get_by_left(&self.problem.first.chars().next().unwrap())
            .unwrap()
            == 0u32
            || *self
                .assignment
                .get_by_left(&self.problem.second.chars().next().unwrap())
                .unwrap()
                == 0u32
            || *self
                .assignment
                .get_by_left(&self.problem.result.chars().next().unwrap())
                .unwrap()
                == 0u32
        {
            self.cost = base.pow(8);
        }

        let mut first_n: u32 = 0;
        for (i, l) in self.problem.first.chars().rev().enumerate() {
            first_n += base.pow(i as u32) * self.assignment.get_by_left(&l).unwrap();
        }

        let mut second_n: u32 = 0;
        for (i, l) in self.problem.second.chars().rev().enumerate() {
            second_n += base.pow(i as u32) * self.assignment.get_by_left(&l).unwrap();
        }

        let mut result_n: u32 = 0;
        for (i, l) in self.problem.result.chars().rev().enumerate() {
            result_n += base.pow(i as u32) * self.assignment.get_by_left(&l).unwrap();
        }

        self.cost = ((result_n as i32) - ((first_n + second_n) as i32)).unsigned_abs();
    }
    pub fn swap_letter_assignment(&mut self, desired: &LetterAssignment) {
        let other_key = *self.assignment.get_by_right(&desired.value).unwrap();
        let current_value = *self.assignment.get_by_left(&desired.letter).unwrap();
        self.assignment.insert(other_key, current_value);
        self.assignment.insert(desired.letter, desired.value);
    }
    pub fn assimilate_hint(&mut self, hint: &Hint) {
        match hint {
            Hint::One(a) => {
                self.swap_letter_assignment(a);
            }
            Hint::Two(a, b) => {
                self.swap_letter_assignment(a);
                self.swap_letter_assignment(b);
            }
            Hint::Three(a, b, c) => {
                self.swap_letter_assignment(a);
                self.swap_letter_assignment(b);
                self.swap_letter_assignment(c);
            }
        }
    }
    pub fn elementary_move(&mut self) {
        let rand_letter = *self
            .assignment
            .iter()
            .map(|(c, _)| *c)
            .collect::<Vec<char>>()
            .choose(&mut rand::thread_rng())
            .unwrap();
        let mut rand_digit = rand::thread_rng().gen_range(0..=9);
        if rand_digit == *self.assignment.get_by_left(&rand_letter).unwrap() {
            rand_digit = (rand_digit + 1) % 10;
        }
        self.swap_letter_assignment(&LetterAssignment {
            letter: rand_letter,
            value: rand_digit,
        });
    }
    pub fn imitate(&mut self, best: &Agent) {
        let letters: Vec<char> = self.problem.letters.iter().cloned().collect();
        let random_letter = letters.choose(&mut rand::thread_rng()).unwrap();
        self.assimilate_hint(&Hint::One(LetterAssignment {
            letter: *random_letter,
            value: *best.assignment.get_by_left(random_letter).unwrap(),
        }));
    }
}

pub fn computational_cost(m: u32, t: f64) -> f64 {
    (m as f64) * t / 3628800.0f64
}

// #[cfg(test)]
// mod tests {
//     use std::vec;

//     use rand::seq::SliceRandom;

//     use crate::{Agent, Hint, LetterAssignment, Problem};

//     #[test]
//     fn problem() {
//         let problem = Problem::default();
//         assert!(problem.first == "donald");
//         assert!(problem.second == "gerald");
//         assert!(problem.result == "robert");
//     }

//     #[test]
//     fn agent() {
//         let problem = Problem::default();
//         let mut agent = Agent::new(&problem);

//         assert!(agent.assignment.len() == 10);
//         let assignment = agent.assignment.clone();
//         agent.assign_random();
//         assert!(agent.assignment != assignment);

//         agent.swap_letter_assignment(&LetterAssignment {
//             letter: 'd',
//             value: 4,
//         });
//         agent.swap_letter_assignment(&LetterAssignment {
//             letter: 'o',
//             value: 5,
//         });

//         assert!(*agent.assignment.get_by_left(&'d').unwrap() == 4);
//         assert!(*agent.assignment.get_by_left(&'o').unwrap() == 5);

//         let assignment = agent.assignment.clone();
//         agent.elementary_move();
//         assert!(agent.assignment != assignment);

//         assert!(!agent.is_solved());

//         let hint = Hint::Two(
//             LetterAssignment {
//                 letter: 'd',
//                 value: 5,
//             },
//             LetterAssignment {
//                 letter: 't',
//                 value: 0,
//             },
//         );

//         agent.assign_random();

//         let mut blackboard = vec![hint.clone()];

//         assert!(blackboard.len() != 0);
//         assert!(blackboard.choose(&mut rand::thread_rng()).unwrap().clone() == hint);
//         assert!(agent.hints.contains(&hint) == false);

//         agent.make_move(&mut blackboard);
//         agent.find_hints();
//         assert!(agent.hints.contains(&hint));

//         agent.assign_random();
//         assert!(!agent.is_solved());
//         agent.assimilate_hint(&Hint::Three(
//             LetterAssignment {
//                 letter: 'a',
//                 value: 4,
//             },
//             LetterAssignment {
//                 letter: 'b',
//                 value: 3,
//             },
//             LetterAssignment {
//                 letter: 'd',
//                 value: 5,
//             },
//         ));
//         agent.assimilate_hint(&Hint::Three(
//             LetterAssignment {
//                 letter: 'e',
//                 value: 9,
//             },
//             LetterAssignment {
//                 letter: 'g',
//                 value: 1,
//             },
//             LetterAssignment {
//                 letter: 'l',
//                 value: 8,
//             },
//         ));
//         agent.assimilate_hint(&Hint::Three(
//             LetterAssignment {
//                 letter: 'n',
//                 value: 6,
//             },
//             LetterAssignment {
//                 letter: 'o',
//                 value: 2,
//             },
//             LetterAssignment {
//                 letter: 'r',
//                 value: 7,
//             },
//         ));
//         agent.assimilate_hint(&Hint::One(LetterAssignment {
//             letter: 't',
//             value: 0,
//         }));
//         assert!(agent.is_solved());
//     }
// }
