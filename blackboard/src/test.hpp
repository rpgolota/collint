#include "rang.hpp"
#include <vector>
#include <string>
#include <iostream>
#include <sstream>

struct TestResult {
    std::string name;
    bool status;
    std::string message;
};

std::vector<TestResult> results;

bool tabulate(std::vector<TestResult> res) {
    
    bool s = true;
    int passed = 0;
    int total = res.size();

    for (int i = 0; i < total; i++) {
        TestResult r = res[i];
        std::string pf;
        if (r.status) {
            pf = "PASS";
            passed++;
            std::cout << rang::fg::green;
        } else {
            pf = "FAIL";
            std::cout << rang::fg::red;
        }
        std::cout << "[" << pf << "] (" << i + 1 << "/" << total << ") " << r.name << (r.status ? " passed. " : " failed. ") << rang::fg::reset << (r.status ? "" : r.message) << std::endl;

        s = s && r.status;
    }
    std::cout << rang::style::bold << rang::fg::blue;
    std::cout << "[DONE] " << total << " tests completed." << std::endl;
    std::cout << (s ? rang::fg::green : rang::fg::red) << "Status: " <<  (s ? passed : total - passed) << (s ? " tests passed." : " tests failed.") << rang::fg::reset << rang::style::reset << std::endl;
    return s;
}

#define TEST(_name, ...) TestResult _name(__VA_ARGS__)
#define START_TESTING int main() { std::vector<TestResult> results;
#define RUN(_name, ...) do { TestResult r = _name(__VA_ARGS__); r.name = #_name; results.push_back(r); } while(false)
#define END_TESTING return tabulate(results); }
#define STATUS(_status, _message) return TestResult {.status = _status, .message = _message}
#define CHECK(_status, _message) do { if (!(_status)) { STATUS(_status, _message); } } while(false)
#define ENDTEST STATUS(true, "")
#define TODO STATUS(false, "Test not yet implemented.")

struct Str {
    Str() = default;
    std::string str() {
        std::string t = b.str();
        b.clear(); 
        return t;
    }
    std::stringstream b;
};
