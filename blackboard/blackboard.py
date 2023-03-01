# Blackboard problem - Roman Golota
# DONALD + GERALD = ROBERT
# find unique digit-to-letter assignment so that the interger numbers represented by the words add upp correctly

import random

# class for solving puzzle
class Cryptarithmetic_Addition:
    
    def __init__(self, a, b, res):
        self.a = str.upper(a)
        self.b = str.upper(b)
        self.res = str.upper(res)
        
        self.letters = "".join(set(a + b + res))
        # initially set to anything
        self.assignment = {l: i+100 for i, l in enumerate(self.letters)}
    
    # randomly give assignment
    def assign_random(self):
        digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
        for l in self.assignment:
            i = random.choice(range(len(digits)))
            self.assignment[l] = digits.pop(i)
    
    # looks through hints
    # hint must match an equation mod 10
    # for all but rightmost row for eps = 0,1 (eps + l1 + l2) mod 10 = l3
    # for rightmost row eps = 0 only
    def find_hints(self):
        N = len(self.a)
        
        hints = []
        
        for i in range(N):
            
            a = self.a[i]
            b = self.b[i]
            r = self.res[i]
            
            a_d = int(self.assignment[a])
            b_d = int(self.assignment[b])
            r_d = int(self.assignment[r])

            # print(f"{a}={a_d},{b}={b_d},{r}={r_d}")

            c1 =  (a_d + b_d) % 10 == r_d
            c2 = i != N-1 and (1 + a_d + b_d) % 10 == r_d
            
            if c1 or c2:
                hint = {a: self.assignment[a], b: self.assignment[b], r: self.assignment[r]}
                hints.append(hint)
            
        return hints
    
    # choses two random letters to swap
    def elementary_move(self):
        a, b = random.choices(self.letters, k=2)
        self.swap_letters(a,b)
    
    # swaps two letter's assignment
    def swap_letters(self, a, b):
        self.assignment[a], self.assignment[b] = self.assignment[b], self.assignment[a]
    
    # assimilate hint
    def assimilate_hint(self, hint):
        to_swap = []
        # for each letter/digit pair in hint, find the letter corresponding to that digit in assignment. Then swap the letters in the assignment
        for l1, d1 in hint.items():
            l2 = None
            for k, v in self.assignment.items():
                if v == d1:
                    l2 = k
                    break
            to_swap.append((l1, l2))
            
        for s in to_swap:
            self.swap_letters(*s)
    
    # check if solved, by converting to numbers and adding
    def is_solved(self):
        a = self.a
        b = self.b
        r = self.res
        
        for l in self.assignment:
            a = a.replace(l, self.assignment[l])
            b = b.replace(l, self.assignment[l])
            r = r.replace(l, self.assignment[l])
            
        a = int(a)
        b = int(b)
        r = int(r)
        
        return a + b == r

# class for handling agent
class Agent:
    
    def __init__(self, a, b, res):
        self.problem = Cryptarithmetic_Addition(a, b, res)
        self.hints = None
    
    # give a random assignment
    def assign_random(self):
        self.problem.assign_random()
    
    # self.find_hints() must be called at least once before calling this method
    def pick_and_replace(self, blackboard):
        
        #? if no hints were found don't do anything
        if len(self.hints) == 0:
            return
        
        # find novel hints not on blackboard
        novel = [h for h in self.hints if h not in blackboard.data]
        #? if no novel hints were found don't do anything
        if not novel:
            return
        # choose a random hint from the novel ones
        selected_hint = random.choice(novel)
        
        # if the blackboard is full, place hint on blackboard replacing a hint that the agent does not have
        if blackboard.is_full():
            # print("Replacing a hint on blackboard.")
            different = [i for i, h in enumerate(blackboard.data) if h not in self.hints]
            to_replace = random.choice(different)
            blackboard.data[to_replace] = selected_hint
        else:
            # print("Appending hint to blackboard.")
            blackboard.data.append(selected_hint)
    
    def find_hints(self):
        # sets self.hints
        self.hints = self.problem.find_hints()
        # if self.hints:
        #     print(f"Found new hints: {len(self.hints)}")
    
    # agent makes a move
    def move(self, blackboard):
        
        # if blackboard is not empty
        if blackboard.data:
            # pick a random hint
            random_hint = random.choice(blackboard.data)
            # if not currently using that hint
            if random_hint not in self.hints:
                # assimilate the hint
                # print("Assimilating hint")
                self.problem.assimilate_hint(random_hint)
                return
        
        # print("Doing elementary move")
        # otherwise make the elementary move
        self.problem.elementary_move()
    
    def is_solved(self):
        return self.problem.is_solved()

# helper class to hold blackboard
class Blackboard:
    
    def __init__(self, B):
        self.b = B
        self.data = []
        
    def is_full(self):
        return len(self.data) == self.b

# blackboard solver. B = Blackboard size, M = group size
def blackboard(M, B, return_agent=False):

    problem = "DONALD", "GERALD", "ROBERT"

    # timestep update delta
    delta = 1.0/float(M)

    blackboard = Blackboard(B)
    agents = [Agent(*problem) for _ in range(M)]

    # at t = 1 all agents' digit-to-letter are selected with equal probability from the pool of 10! valid assignments
    t = 1.0

    for a in agents:
        # initial digit to letter assignment
        a.assign_random()
        # find inital hints
        a.find_hints()
        # update blackboard
        a.pick_and_replace(blackboard)
        
    matchmax = 0
    while True:
        # choose random agent
        a = random.choice(agents)
        # perform a move after looking over blackbloard
        a.move(blackboard)
        # update hints
        a.find_hints()
        # update t
        t += delta
        # update blackboard
        a.pick_and_replace(blackboard)
        # check for solution
        if a.is_solved():
            print(f"Max Matching({matching(a.problem.assignment)}), C({comp_cost(M, t):.6f})")
            if return_agent:
                return t, a
            else:
                return t
            
        mat = matching(a.problem.assignment)
        if mat > matchmax:
            matchmax = mat 
        print(f"Max Matching({matchmax}), C({comp_cost(M, t):.6f})")

# computes the computational cost
def comp_cost(M, t_star):
    return (float(M) * t_star) / 3628800.0

# helper function to ouput the matching assignments to real solution
def matching(assignment):
    real = {"A": "4", "B": "3", "D": "5", "E": "9", "G": "1", "L": "8", "N": "6", "O": "2", "R": "7", "T": "0"}
    matching = 0
    for l in real:
        if assignment[l] == real[l]:
            matching += 1
    return matching

# sample running script
if __name__ == "__main__":
    
    M = 10
    B = 7
    show_assignment = True
    
    t_star, a = blackboard(M, B, return_agent=show_assignment)
    C = comp_cost(M, t_star)
    
    print("----------------------------")
    print(f"Solved puzzle with t* = {t_star}, and C = {C}")
    if show_assignment:
        print(f"Final assignemnt: {a.problem.assignment}")