from collint import blackboard, imitative

# General parameters
RUN_TYPE = "imitative" # blackboard | imitative
MAX_C = 10.0
M = [8,9,10]
N = 10

# Blackboard parameters
B = [6,7,8]
COMPUTE_PHI = False

# Imitative parameters
P = [0.5, 0.6]

if RUN_TYPE == "blackboard":
    for m in M:
        for b in B:
            for _ in range(N):
                result = blackboard(m, b, max_c=MAX_C, compute_phi=COMPUTE_PHI)
                if result is not None:
                    if COMPUTE_PHI:
                        print(f"{result['m']},{result['b']},{result['t_star']},{result['phi']},{result['c']}")
                    else:
                        print(f"{result['m']},{result['b']},{result['t_star']},{result['c']}")
elif RUN_TYPE == "imitative":
    for m in M:
        for p in P:
            for _ in range(N):
                result = imitative(m, p, max_c=MAX_C)
                if result is not None:
                    print(f"{result['m']},{result['p']},{result['t_star']},{result['c']}")