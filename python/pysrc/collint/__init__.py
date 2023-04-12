from .wrapper import imitative, blackboard


__doc__ = wrapper.__doc__
if hasattr(wrapper, "__all__"):
    __all__ = wrapper.__all__