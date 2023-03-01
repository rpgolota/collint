#include "test.hpp"
#include "blackboard.h"

BlackboardMethod::Agent make_problem_agent() {
    return BlackboardMethod::Agent("DONALD", "GERALD", "ROBERT");
}

void assign_solution(BlackboardMethod::Agent& a) {
    a.assignment['A'] = '4';
    a.assignment['B'] = '3';
    a.assignment['D'] = '5';
    a.assignment['E'] = '9';
    a.assignment['G'] = '1';
    a.assignment['L'] = '8';
    a.assignment['N'] = '6';
    a.assignment['O'] = '2';
    a.assignment['R'] = '7';
    a.assignment['T'] = '0';
}

TEST(find_hints) {

    Str s;

    BlackboardMethod::Agent a = make_problem_agent();
    assign_solution(a);
    a.find_hints();
    // a.print();

    s.b << "The solution shows " << a.hints.size() << ", not 6.";
    CHECK(a.hints.size() == 6, s.str());

    a.assignment['A'] = '9';
    a.assignment['E'] = '4';
    a.find_hints();
    // a.print();

    s.b << "Swapping A and E should decrease hints to 4. Hints = " << a.hints.size();
    CHECK(a.hints.size() == 4, s.str());

    ENDTEST;
}

TEST(is_solved) {

    Str s;

    BlackboardMethod::Agent a = make_problem_agent();
    assign_solution(a);

    s.b << "The valid solution is not shown as solved.";
    CHECK(a.is_solved(), s.str());

    a.assignment['A'] = '9';
    a.assignment['E'] = '4';

    s.b << "An invalid solution is shown as solved.";
    CHECK(!a.is_solved(), s.str());

    ENDTEST;
}

TEST(make_elementary_move) {
    TODO;
}

TEST(assimilate_hint) {
    TODO;
}

TEST(make_move) {
    TODO;
}

TEST(update_blackboard) {
    TODO;
}

TEST(make_random_assignment) {
    TODO;
}

START_TESTING

    RUN(find_hints);
    RUN(is_solved);
    RUN(make_elementary_move);
    RUN(assimilate_hint);
    RUN(make_move);
    RUN(update_blackboard);
    RUN(make_random_assignment);

END_TESTING