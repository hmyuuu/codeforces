__author__ = 'ius'

from sys import stdin
input = stdin.readline

"""
4A. Watermelon

https://codeforces.com/problemset/problem/4/A

You are given a positive integer w (w <= 100) that represents the weight of a watermelon.
You need to check if the watermelon can be divided into two parts, each of which is a positive integer and the sum of the two parts is equal to w.
If it can, print "YES", otherwise print "NO".

Input:
The first line of the input contains a single integer w (1 <= w <= 100).

Output:
Print "YES" if the watermelon can be divided into two parts, each of which is a positive integer and the sum of the two parts is equal to w, otherwise print "NO".

Example:
Input:
8
Output:
YES

"""
w = int(input().strip())
if w > 2 and w % 2 == 0: # w is even and greater than 2
    print("YES")
else:
    print("NO")
