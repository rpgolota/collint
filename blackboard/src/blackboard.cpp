#include "blackboard.h"
#include "random.h"
#include <iostream>
#include <ctype.h>
#include <algorithm>


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

    for (auto& a : agents) {
        a.make_random_assignment();
        a.find_hints();
        a.update_blackboard(blackboard, B);
    }

    while (true) {

        Agent a = *select_randomly(agents.begin(), agents.end());

        a.make_move(blackboard);
        a.find_hints();
        t += delta_t;
        a.update_blackboard(blackboard, B);

        // for (auto a : blackboard) {
        //     std::cout << "[" << std::get<0>(a).first << "=" << std::get<0>(a).second << ", " << std::get<1>(a).first << "=" << std::get<1>(a).second << ", " << std::get<2>(a).first << "=" << std::get<2>(a).second << "] ";
        // }
        // if (!blackboard.empty()) {
        //     std::cout << std::endl;
        // }

        if (a.is_solved()) {
            return t;
        }
    }
}

/// AGENT

// private

void BlackboardMethod::Agent::assimilate_hint(Hint hint) {

    One a1 = std::get<0>(hint);
    One a2 = std::get<1>(hint);
    One a3 = std::get<2>(hint);

    std::vector<One> new_hint;
    new_hint.push_back(a1);
    if (a2 != a1) {
        new_hint.push_back(a2);
    }
    if (a3 != a2 && a3 != a1) {
        new_hint.push_back(a3);
    }

    std::vector<One> to_swap;
    for (auto h : new_hint) {
        for (auto a : assignment) {
            if (a.second == h.second) {
                to_swap.push_back(std::make_pair(h.first, a.first));
                break;
            }
        }
    }

    for (auto s : to_swap) {
        char t = assignment[s.first];
        assignment[s.first] = assignment[s.second];
        assignment[s.second] = t;
    }

}

void BlackboardMethod::Agent::make_elementary_move() {

    One one = *select_randomly(assignment.begin(), assignment.end());
    One two = *select_randomly(assignment.begin(), assignment.end());

    if (one == two) {
        make_elementary_move();
    }

    char t = one.second;
    assignment[one.first] = two.second;
    assignment[two.first] = t;
}

//public

BlackboardMethod::Agent::Agent(std::string A, std::string B, std::string C) {
    problem[0] = A;
    problem[1] = B;
    problem[2] = C;

    std::string letters;
    for (auto c : A) {
        if (std::find(letters.begin(), letters.end(), c) ==  letters.end()) {
            letters.push_back(c);
        }
    }
    for (auto c : B) {
        if (std::find(letters.begin(), letters.end(), c) ==  letters.end()) {
            letters.push_back(c);
        }
    }
    for (auto c : C) {
        if (std::find(letters.begin(), letters.end(), c) ==  letters.end()) {
            letters.push_back(c);
        }
    }

    for (auto c : letters) {
        assignment.insert({c, '*'});
    }

}

void BlackboardMethod::Agent::print() {
    std::cout << "Assignment: ";
    for (auto a : assignment) {
        std::cout << "(" << a.first << ", " << a.second << ") ";
    }
    std::cout << std::endl;
    std::cout << "Hints: ";
    for (auto h : hints) {
        std::cout << "(" << std::get<0>(h).first << "=" << std::get<0>(h).second << ", " << std::get<1>(h).first << "=" << std::get<1>(h).second << ", " << std::get<2>(h).first << "=" << std::get<2>(h).second << ") ";
    }
    std::cout << std::endl;
    
}

void BlackboardMethod::Agent::make_random_assignment() {

    std::string digits = "0123456789";

    for (auto it : assignment) {
        char d = *select_randomly(digits.begin(), digits.end());
        assignment[it.first] = d;
        digits.erase(std::remove(digits.begin(), digits.end(), d), digits.end());
    }
}
void BlackboardMethod::Agent::find_hints() {

    hints.clear();

    for (int i = 0; i < problem[0].size(); i++) {

        char a_letter = problem[0][i];
        char b_letter = problem[1][i];
        char c_letter = problem[2][i];

        char a_digit_c = assignment[a_letter];
        char b_digit_c = assignment[b_letter];
        char c_digit_c = assignment[c_letter];

        int a_digit = a_digit_c - 48;
        int b_digit = b_digit_c - 48;
        int c_digit = c_digit_c - 48;

        // std::cout << a_digit << " + " << b_digit << " = " << (a_digit + b_digit) % 10 << "(" << c_digit << ")" << std::endl;
        // std::cout << a_digit << " + " << b_digit << " + 1" << " = " << (a_digit + b_digit + 1) % 10 << "(" << c_digit << ")" << std::endl;

        bool cond1 = (a_digit + b_digit) % 10 == c_digit;
        bool cond2 = i != (problem[0].size() - 1) && (a_digit + b_digit + 1) % 10 == c_digit;

        if (cond1 || cond2) {

            Hint hint = std::make_tuple(std::make_pair(a_letter, a_digit_c), std::make_pair(b_letter, b_digit_c), std::make_pair(c_letter, c_digit_c));
            hints.push_back(hint);
        }
    }

}
void BlackboardMethod::Agent::update_blackboard(std::vector<BlackboardMethod::Hint>& blackboard, int B) {
    
    if (hints.empty()) {
        // std::cout << "No hints" << std::endl;
        return;
    }

    std::vector<Hint> novel;
    for (auto h : hints) {
        if (std::find(blackboard.begin(), blackboard.end(), h) == blackboard.end()) {
            novel.push_back(h);
        }
    }

    if (novel.empty()) {
        // std::cout << "No novel" << std::endl;
        return;
    }

    Hint selected = *select_randomly(novel.begin(), novel.end());

    if (blackboard.size() == B) {
        // std::cout << "Full blackboard" << std::endl;
        std::vector<int> different;
        for (auto h : blackboard) {
            auto it = std::find(hints.begin(), hints.end(), h);
            if (it == hints.end()) {
                different.push_back((int)(it - hints.begin()));
            }
        }
        int to_replace = *select_randomly(different.begin(), different.end());
        blackboard[to_replace] = selected;
    } else {
        // std::cout << "Not full blackboard" << std::endl;
        blackboard.push_back(selected);
    }

}
void BlackboardMethod::Agent::make_move(std::vector<BlackboardMethod::Hint>& blackboard) {

    if (!blackboard.empty()) {
        Hint random = *select_randomly(blackboard.begin(), blackboard.end());
        if (std::find(hints.begin(), hints.end(), random) == hints.end()) {
            assimilate_hint(random);
            return;
        }
    }
    make_elementary_move();
}
bool BlackboardMethod::Agent::is_solved() {

    std::string A = problem[0];
    std::string B = problem[1];
    std::string C = problem[2];
    

    for (auto a : assignment) {
        std::replace(A.begin(), A.end(), a.first, a.second);
        std::replace(B.begin(), B.end(), a.first, a.second);
        std::replace(C.begin(), C.end(), a.first, a.second);
    }

    int a = std::atoi(A.c_str());
    int b = std::atoi(B.c_str());
    int c = std::atoi(C.c_str());

    // std::cout << a << " + " << b << " = " << a + b << "(" << c << ")" << std::endl;

    return a + b == c;
}