/*
71A. Way Too Long Words

https://codeforces.com/problemset/problem/71/A

Input:
The first line contains an integer n (1 ≤ n ≤ 100). Each of the following n lines contains one word. All words contain only lowercase Latin letters and have lengths from 1 to 100.

Output:
Print n lines. The i-th line should contain the result of replacing the i-th word from the input.

Example:
Input:
4
word
localization
internationalization
pneumonoultramicroscopicsilicovolcanoconiosis

Output:
word
l10n
i18n
p43s
*/

#include <iostream>
using namespace std;

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    
    int n;
    cin >> n;
    
    for (int i = 0; i < n; i++) {
        string word;
        cin >> word;
        
        if (word.length() <= 10) {
            cout << word << endl;
        } else {
            cout << word[0] << word.length() - 2 << word[word.length() - 1] << endl;
        }
    }
    
    return 0;
}
