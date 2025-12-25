__author__ = 'ius'

from sys import stdin
input = stdin.readline

"""
231A. Team

https://codeforces.com/problemset/problem/231/A

Input:
The first line contains a single integer n (1 <= n <= 1000) - the number of problems in the contest.

The next n lines contain three integers each, each integer is either 0 or 1. If the first number in the line is 1, then the second number is the result of the first problem, and the third number is the result of the second problem.

Output:
Print a single integer - the number of problems the friends will implement on the contest.

Example:
Input:

3
1 1 0
1 1 1
1 0 0

Output:
2
"""

n = int(input().strip())
count = 0
for _ in range(n):
    p, v, t = map(int, input().strip().split())
    if p + v + t >= 2:
        count += 1
print(count)