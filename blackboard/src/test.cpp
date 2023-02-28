#include "test.hpp"
#include "blackboard.h"

TEST(find_hints) {

    bool status = true;
    std::string fail_message;

    BlackboardMethod::Agent a("DONALD", "GERALD", "ROBERT");
    a.make_random_assignment();
    a.assignment['N'] = '1';
    a.assignment['R'] = '4';
    a.assignment['B'] = '5';
    a.find_hints();

    status = status && (a.hints.size() >= 1);

    STATUS(status, fail_message);
}

START_TESTING

    RUN(find_hints);

END_TESTING