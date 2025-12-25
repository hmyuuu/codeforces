
/*
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
*/

#include <iostream>
using namespace std;

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    
    int n;
    cin >> n;
    
    int count = 0;
    for (int i = 0; i < n; i++) {
        int p, v, t;
        cin >> p >> v >> t;
        
        if (p + v + t >= 2) {
            count++;
        }
    }
    
    cout << count << endl;
    
    return 0;
}
