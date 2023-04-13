COLLINT_PYTHON_CHECK_TYPES = True


def disable_python_type_checking():
    global COLLINT_PYTHON_CHECK_TYPES
    COLLINT_PYTHON_CHECK_TYPES = False


def enable_python_type_checking():
    global COLLINT_PYTHON_CHECK_TYPES
    COLLINT_PYTHON_CHECK_TYPES = True


def is_python_type_checking_enabled():
    return COLLINT_PYTHON_CHECK_TYPES
