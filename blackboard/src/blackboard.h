#ifndef BLACKBOARD_H
#define BLACKBOARD_H

#include <string>
#include <vector>
#include <utility>

template<typename Iter, typename RandomGenerator>
Iter select_randomly(Iter, Iter, RandomGenerator&);

template<typename Iter>
Iter select_randomly(Iter, Iter);

class BlackboardMethod {

public:

    BlackboardMethod(int, int, std::string);
    double solve();

    using Hint = std::pair<char, char>;

    class Agent {
        void swap_letter_assignments(char, char);
        void make_elementary_move();

    public:

        Agent(std::string, std::string, std::string);

        void make_random_assignment();
        void find_hints();
        void update_blackboard(std::vector<Hint>&);
        void make_move(std::vector<Hint>&);
        bool is_solved();

        std::string problem[3];
        char letters[10];
        char assignment[10];
        std::vector<Hint> hints;
    };

    int M, B;
    std::string problem_statement;
    std::vector<Agent> agents;
    std::vector<Hint> blackboard;
};

#endif//BLACKBOARD_H