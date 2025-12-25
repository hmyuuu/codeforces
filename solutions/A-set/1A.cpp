/*
1A. Theatre Square

https://codeforces.com/problemset/problem/1/A

Input:
The first line contains three integers n, m and a (1 <= n, m, a <= 10^9).

Output:
Print a single integer - the number of squares that will not be covered by the carpet.
*/

#include <iostream>
using namespace std;

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    
    long long n, m, a;
    cin >> n >> m >> a;
    
    long long tiles_n = (n + a - 1) / a;
    long long tiles_m = (m + a - 1) / a;
    
    cout << tiles_n * tiles_m << endl;
    
    return 0;
}
