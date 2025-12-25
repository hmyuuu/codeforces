__author__ = 'ius'

from sys import stdin
input = stdin.readline

"""
1A. Theatre Square

https://codeforces.com/problemset/problem/1/A

Input:
The first line contains three integers n, m and a (1 <= n, m, a <= 10^9).

Output:
Print a single integer - the number of squares that will not be covered by the carpet.
"""
from math import ceil
n, m, a = map(int, input().strip().split())
print(ceil(n / a) * ceil(m / a))
