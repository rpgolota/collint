#include "blackboard.h"

#include <iostream>
#include <iomanip>
#include <fstream>

double computational_cost(int, double);
double blackboard(int, int);

double computational_cost(int M, double t_star) {
    return ((double)M * t_star) / 3628800.0;
}

int main() {

    int return_code = 0;
    std::string problem_statement = "DONALD + GERALD = ROBERT";

    std::ofstream output;
    output.open("out.csv");

    output << "Group Size" << "," << "Blackboard Size" << "," << "Time when Completed" << "," << "Computational Cost" << std::endl;
    
    /* Run some kind of loop here */ {

        int M = 10;
        int B = 7;

        double t_star = BlackboardMethod(M, B, problem_statement).solve();
        double C = computational_cost(M, t_star);
        output << M << "," << B << "," << std::fixed << std::setprecision(2) << t_star << "," << std::setprecision(6) << C << std::endl;
    
    }

    output.close();
    
    return return_code;
}