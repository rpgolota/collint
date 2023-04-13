from . import imitative, blackboard, config
from itertools import product


class Experiment:
    def __init__(self):
        self._parallel = False
        self._type_checking = True
        self._max_c = 10.0

        self._run_type = None

    def disable_type_checking(self):
        self._type_checking = False
        return self

    def max_c(self, max_c):
        self._max_c = max_c
        return self

    def parallel(self, show_progress=True):
        self._parallel = (True,)
        self._show_progress = show_progress
        return self

    def blackboard(self, ms, bs, n, compute_phi=False):
        if self._run_type:
            raise ValueError("Already chose a run type")
        self._run_type = "blackboard"
        self._ms = ms
        self._bs = bs
        self._n = n
        self._compute_phi = compute_phi
        return self

    def imitative(self, ms, ps, n):
        if self._run_type:
            raise ValueError("Already chose a run type")
        self._run_type = "imitative"
        self._ms = ms
        self._ps = ps
        self._n = n
        return self

    def _run_parallel(self):
        if self._run_type == "blackboard":
            return blackboard.blackboard_parallel(
                self._ms,
                self._bs,
                self._n,
                max_c=self._max_c,
                compute_phi=self._compute_phi,
                show_progress=self._show_progress,
            )
        else:
            return imitative.imitative_parallel(
                self._ms,
                self._ps,
                self._n,
                max_c=self._max_c,
                show_progress=self._show_progress,
            )

    def _yield_non_parallel(self):
        if self._run_type == "blackboard":
            for m, b, _ in product(self._ms, self._bs, range(self._n)):
                yield blackboard.blackboard(
                    m, b, max_c=self._max_c, compute_phi=self._compute_phi
                )
        else:
            for m, p, _ in product(self._ms, self._ps, range(self._n)):
                yield imitative.imitative(m, p, max_c=self._max_c)
        if not self._type_checking and self._type_checking_original:
            config.enable_python_type_checking()

    # if parallel, runs fully and returns results
    # if not parallel, gives a generator that yields value by value. call list() on result to run all
    def run(self):
        self._type_checking_original = config.is_python_type_checking_enabled()
        if self._run_type is None:
            raise ValueError(
                "Cannot run without calling either blackboard or imitative method"
            )
        if not self._type_checking:
            config.disable_python_type_checking()

        if self._parallel:
            results = self._run_parallel()
            if not self._type_checking and self._type_checking_original:
                config.enable_python_type_checking()
            return results
        else:
            return self._yield_non_parallel()
