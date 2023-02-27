#ifndef RANDOM_H
#define RANDOM_H

#include <random>
#include <iterator>
#include <chrono>

template<typename Iter, typename RandomGenerator>
Iter select_randomly(Iter start, Iter end, RandomGenerator& g) {
    std::uniform_int_distribution<> dis(0, std::distance(start, end) - 1);
    std::advance(start, dis(g));
    return start;
}

template<typename Iter>
Iter select_randomly(Iter start, Iter end) {
    // static std::random_device rd;
    static std::mt19937 gen(std::chrono::high_resolution_clock::now().time_since_epoch().count());
    return select_randomly(start, end, gen);
}


#endif//RANDOM_H