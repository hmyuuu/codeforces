__author__ = 'ius'

from sys import stdin
input = stdin.readline
"""
71A. Way Too Long Words

https://codeforces.com/problemset/problem/71/A

**Input**

The first line contains an integer n (1 ≤ n ≤ 100). Each of the following n lines contains one word. All words contain only lowercase Latin letters and have lengths from 1 to 100.

**Output**

Print n lines. The i-th line should contain the result of replacing the i-th word from the input.

**Example**

Input

4
word
localization
internationalization
pneumonoultramicroscopicsilicovolcanoconiosis

Output

word
l10n
i18n
p43s
"""

# Solution here
n = int(input().strip())
for _ in range(n):
    word = input().strip()
    if len(word) <= 10:
        print(word)
    else:
        print(f"{word[0]}{len(word) - 2}{word[-1]}")