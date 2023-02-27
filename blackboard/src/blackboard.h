#ifndef BLACKBOARD_H
#define BLACKBOARD_H

#include <string>
#include <vector>
#include <utility>
#include <tuple>
#include <unordered_map>

class BlackboardMethod {

public:

    BlackboardMethod(int, int, std::string);
    double solve();

    using One = std::pair<char, char>;
    using Hint = std::tuple<One, One, One>;

    class Agent {
        void make_elementary_move();
        void assimilate_hint(Hint);

    public:

        Agent(std::string, std::string, std::string);

        void print();
        void make_random_assignment();
        void find_hints();
        void update_blackboard(std::vector<Hint>&, int);
        void make_move(std::vector<Hint>&);
        bool is_solved();

        std::string problem[3];
        std::unordered_map<char, char> assignment;
        std::vector<Hint> hints;
    };

    int M, B;
    std::string problem_statement;
    std::vector<Agent> agents;
    std::vector<Hint> blackboard;
};

#endif//BLACKBOARD_H