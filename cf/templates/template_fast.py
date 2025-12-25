__author__ = 'ius'

import sys

data = sys.stdin.buffer.read().split()
it = iter(data)

def next_token() -> bytes:
    return next(it)


def next_int() -> int:
    return int(next_token())


def solve():
    # Solution here
    pass


def main():
    t = next_int()
    for _ in range(t):
        solve()


if __name__ == "__main__":
    main()
