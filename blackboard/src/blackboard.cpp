#include "blackboard.h"
#include <iostream>
#include <ctype.h>
#include <random>
#include <iterator>

template<typename Iter, typename RandomGenerator>
Iter select_randomly(Iter start, Iter end, RandomGenerator& g) {
    std::uniform_int_distribution<> dis(0, std::distance(start, end) - 1);
    std::advance(start, dis(g));
    return start;
}

template<typename Iter>
Iter select_randomly(Iter start, Iter end) {
    static std::random_device rd;
    static std::mt19937 gen(rd());
    return select_randomly(start, end, gen);
}

/// BLACKBOARDMETHOD

// private

// - none -

// public

BlackboardMethod::BlackboardMethod(int M, int B, std::string p) : M(M), B(B), problem_statement(p) {
    agents.reserve(M);
    blackboard.reserve(B);

    std::size_t plus = p.find('+');
    std::size_t equals = p.find('=');

    std::string sA, sB, sC;

    bool invalid = plus == std::string::npos || equals == std::string::npos;
    if (invalid) {
        std::cout << "Invalid problem statement. Setting to default." << std::endl;
        sA.assign("DONALD");
        sB.assign("GERALD");
        sC.assign("ROBERT");
    }

    std::size_t current = 0;
    while (current < plus) {
        if (p[current] >= 'A' && p[current] <= 'Z')
            sA.push_back(p[current]);

        current++;
    }

    while (current < equals) {
        if (p[current] >= 'A' && p[current] <= 'Z')
            sB.push_back(p[current]);

        current++;
    }

    while (current < p.length()) {
        if (p[current] >= 'A' && p[current] <= 'Z')
            sC.push_back(p[current]);
            
        current++;
    }

    for (int i = 0; i < M; i++) {
        agents.emplace_back(sA, sB, sC);
    }
}

double BlackboardMethod::solve() {

    double delta_t = 1.0/(double)M;
    double t = 1.0;

    for (auto a : agents) {
        a.make_random_assignment();
        a.find_hints();
        a.update_blackboard(blackboard);
    }

    while (false) {

        Agent a = *select_randomly(agents.begin(), agents.end());
        a.make_move(blackboard);
        a.find_hints();
        t += delta_t;
        a.update_blackboard(blackboard);

        if (a.is_solved()) {
            return t;
        }
    }

    return 0.0;
}

/// AGENT

// private

void BlackboardMethod::Agent::swap_letter_assignments(char, char) {

}
void BlackboardMethod::Agent::make_elementary_move() {

}

//public

BlackboardMethod::Agent::Agent(std::string A, std::string B, std::string C) {
    problem[0] = A;
    problem[1] = B;
    problem[2] = C;
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