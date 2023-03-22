use bimap::BiHashMap;
use rand::{seq::SliceRandom, Rng};
use std::collections::HashSet;

#[derive(Debug)]
struct Problem {
    first: String,
    second: String,
    result: String,
    letters: HashSet<char>,
}

impl Problem {
    fn new(first: &str, second: &str, result: &str) -> Self {
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
struct LetterAssignment {
    letter: char,
    value: u32,
}

#[derive(Debug, Clone, Copy)]
enum Hint {
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

#[derive(Debug, Clone)]
struct Agent<'a> {
    problem: &'a Problem,
    hints: Vec<Hint>,
    assignment: BiHashMap<char, u32>,
}

impl<'a> Agent<'a> {
    fn new(problem: &'a Problem) -> Self {
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
        }
    }
}

impl<'a> Agent<'a> {
    fn assign_random(&mut self) {
        let mut digits: Vec<u32> = (0..=9).collect();
        for (l, _) in self.assignment.clone().iter() {
            let to_remove = rand::thread_rng().gen_range(0..digits.len());
            let random = digits.remove(to_remove);
            self.assignment.insert(*l, random);
        }
    }
    fn find_hints(&mut self) {
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
    fn pick_and_replace(&mut self, blackboard: &mut Vec<Hint>) {
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
    fn make_move(&mut self, blackboard: &mut Vec<Hint>) {
        if !blackboard.is_empty() {
            let random_hint = blackboard.choose(&mut rand::thread_rng()).unwrap();
            if !self.hints.contains(random_hint) {
                self.assimilate_hint(random_hint);
                return;
            }
        }
        self.elementary_move();
    }
    fn is_solved(&self) -> bool {
        let mut first = self.problem.first.clone();
        let mut second = self.problem.second.clone();
        let mut result = self.problem.result.clone();

        for (l, v) in self.assignment.iter() {
            first = first.replace(*l, (*v).to_string().as_str());
            second = second.replace(*l, (*v).to_string().as_str());
            result = result.replace(*l, (*v).to_string().as_str());
        }

        let first = first.parse::<u32>().unwrap();
        let second = second.parse::<u32>().unwrap();
        let result = result.parse::<u32>().unwrap();

        first + second == result
    }
    fn swap_letter_assignment(&mut self, desired: &LetterAssignment) {
        let other_key = *self.assignment.get_by_right(&desired.value).unwrap();
        let current_value = *self.assignment.get_by_left(&desired.letter).unwrap();
        self.assignment.insert(other_key, current_value);
        self.assignment.insert(desired.letter, desired.value);
    }
    fn assimilate_hint(&mut self, hint: &Hint) {
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
    fn elementary_move(&mut self) {
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
}

#[derive(Debug)]
pub struct BlackboardResult {
    pub m: u32,
    pub b: u32,
    pub t_star: f64,
}

impl ToString for BlackboardResult {
    fn to_string(&self) -> String {
        format!("{},{},{}", self.m, self.b, self.t_star)
    }
}

pub fn blackboard(b: u32, m: u32) -> Option<BlackboardResult> {
    let problem = Problem::new("DONALD", "GERALD", "ROBERT");

    let delta = 1.0 / (m as f64);
    let mut blackboard: Vec<Hint> = Vec::new();
    blackboard.reserve(b as usize);
    let mut agents = Vec::new();
    for _ in 0..m {
        agents.push(Agent::new(&problem));
    }

    let mut t = 1.0;

    for a in agents.iter_mut() {
        a.assign_random();
        a.find_hints();
        a.pick_and_replace(&mut blackboard);
    }

    loop {
        let a = agents.choose_mut(&mut rand::thread_rng()).unwrap();
        a.make_move(&mut blackboard);
        a.find_hints();
        t += delta;
        a.pick_and_replace(&mut blackboard);
        if a.is_solved() {
            return Some(BlackboardResult { m, b, t_star: t });
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use rand::seq::SliceRandom;

    use crate::{Agent, Hint, LetterAssignment, Problem};

    #[test]
    fn problem() {
        let problem = Problem::default();
        assert!(problem.first == "donald");
        assert!(problem.second == "gerald");
        assert!(problem.result == "robert");
    }

    #[test]
    fn agent() {
        let problem = Problem::default();
        let mut agent = Agent::new(&problem);

        assert!(agent.assignment.len() == 10);
        let assignment = agent.assignment.clone();
        agent.assign_random();
        assert!(agent.assignment != assignment);

        agent.swap_letter_assignment(&LetterAssignment {
            letter: 'd',
            value: 4,
        });
        agent.swap_letter_assignment(&LetterAssignment {
            letter: 'o',
            value: 5,
        });

        assert!(*agent.assignment.get_by_left(&'d').unwrap() == 4);
        assert!(*agent.assignment.get_by_left(&'o').unwrap() == 5);

        let assignment = agent.assignment.clone();
        agent.elementary_move();
        assert!(agent.assignment != assignment);

        assert!(!agent.is_solved());

        let hint = Hint::Two(
            LetterAssignment {
                letter: 'd',
                value: 5,
            },
            LetterAssignment {
                letter: 't',
                value: 0,
            },
        );

        agent.assign_random();

        let mut blackboard = vec![hint.clone()];

        assert!(blackboard.len() != 0);
        assert!(blackboard.choose(&mut rand::thread_rng()).unwrap().clone() == hint);
        assert!(agent.hints.contains(&hint) == false);

        agent.make_move(&mut blackboard);
        agent.find_hints();
        assert!(agent.hints.contains(&hint));

        agent.assign_random();
        assert!(!agent.is_solved());
        agent.assimilate_hint(&Hint::Three(
            LetterAssignment {
                letter: 'a',
                value: 4,
            },
            LetterAssignment {
                letter: 'b',
                value: 3,
            },
            LetterAssignment {
                letter: 'd',
                value: 5,
            },
        ));
        agent.assimilate_hint(&Hint::Three(
            LetterAssignment {
                letter: 'e',
                value: 9,
            },
            LetterAssignment {
                letter: 'g',
                value: 1,
            },
            LetterAssignment {
                letter: 'l',
                value: 8,
            },
        ));
        agent.assimilate_hint(&Hint::Three(
            LetterAssignment {
                letter: 'n',
                value: 6,
            },
            LetterAssignment {
                letter: 'o',
                value: 2,
            },
            LetterAssignment {
                letter: 'r',
                value: 7,
            },
        ));
        agent.assimilate_hint(&Hint::One(LetterAssignment {
            letter: 't',
            value: 0,
        }));
        assert!(agent.is_solved());
    }
}
