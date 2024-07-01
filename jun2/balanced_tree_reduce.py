def balanced_tree_reduce(func, iterable, initial=None):
    """
    Reduces an iterable in a balanced tree-like fashion.

    Args:
    func: A function that takes two arguments and returns a single value.
    iterable: The iterable to reduce.
    initial: An optional initial value. If provided, it's treated as the first element in the reduction.

    Returns:
    The final reduced value.
    """
    items = list(iterable)

    if not items:
        if initial is not None:
            return initial
        raise ValueError("balanced_tree_reduce() of empty sequence with no initial value")

    if initial is not None:
        items.insert(0, initial)

    while len(items) > 1:
        new_items = []
        for i in range(0, len(items) - 1, 2):
            new_items.append(func(items[i], items[i + 1]))
        if len(items) % 2 != 0:
            new_items.append(items[-1])
        items = new_items

    return items[0]


# Example usage
numbers = [1, 2, 3, 4, 5, 6, 7, 8]
result = balanced_tree_reduce(lambda x, y: x + y, numbers)
print(result)  # Output: 36

# Example with initial value
result_with_initial = balanced_tree_reduce(lambda x, y: x + y, numbers, initial=10)
print(result_with_initial)  # Output: 46
