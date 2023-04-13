from collint import Experiment


def run_blackboard_experiment():
    M = range(7, 10 + 1)
    B = range(6, 8 + 1)
    N = 1

    results = (
        Experiment()
        .disable_type_checking()
        .max_c(10.0)
        .parallel(show_progress=True)
        .blackboard(M, B, N, compute_phi=False)
        .run()
    )
    return results


def run_imitative_experiment():
    M = range(7, 10 + 1)
    P = [0.5, 0.6]
    N = 1

    results = (
        Experiment()
        .disable_type_checking()
        .max_c(10.0)
        .parallel(show_progress=True)
        .imitative(M, P, N)
        .run()
    )
    return results


if __name__ == "__main__":
    rb = run_blackboard_experiment()
    ri = run_imitative_experiment()

    print("Blackboard results:")
    for r in rb:
        print(r)
    print()
    print("Imitative Results:")
    for r in ri:
        print(r)
