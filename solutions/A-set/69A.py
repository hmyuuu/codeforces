__author__ = 'ius'

"""
69A. Young Physicist

https://codeforces.com/problemset/problem/69/A

Input:
The first line contains a single integer n (1 <= n <= 100) - the number of forces.

Output:
Print "YES" if the forces are in equilibrium, otherwise print "NO".
"""

from sys import stdin
input = stdin.readline

n = int(input().strip())
x = 0
y = 0
z = 0
for _ in range(n):
    dx, dy, dz = map(int, input().strip().split())
    x += dx
    y += dy
    z += dz
if x == 0 and y == 0 and z == 0:
    print("YES")
else:
    print("NO")
