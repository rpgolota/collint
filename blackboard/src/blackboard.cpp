#include "blackboard.h"

/// BLACKBOARDMETHOD

// private

// - none -

// public

BlackboardMethod::BlackboardMethod(int M, int B) {

}

double BlackboardMethod::solve() {
    return 0.0;
}

/// AGENT

// private

void BlackboardMethod::Agent::swap_letter_assignments(char, char) {

}
void BlackboardMethod::Agent::make_elementary_move() {

}
void BlackboardMethod::Agent::parse_problem_statement(std::string) {

}

//public

BlackboardMethod::Agent::Agent(std::string problem_statement) {
    parse_problem_statement(problem_statement);
}

void BlackboardMethod::Agent::make_random_assignment() {

}
void BlackboardMethod::Agent::find_hints() {

}
void BlackboardMethod::Agent::update_blackboard(std::vector<BlackboardMethod::Hint>&) {

}
void BlackboardMethod::Agent::make_move(std::vector<BlackboardMethod::Hint>&) {

}
bool BlackboardMethod::Agent::is_solved() {
    return false;
}