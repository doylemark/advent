#include <iostream>
#include <set>
#include <string>
#include <utility>
#include <vector>
using namespace std;

int main() {
  int H = 130;

  vector<string> a(H);

  for (int i = 0; i < H; i++) {
    cin >> a[i];
  }

  int W = a[0].length();

  pair<int, int> cursor{-1, -1};

  for (int row = 0; row < H; row++) {
    for (int col = 0; col < W; col++) {
      if (a[row][col] == '^') {
        cursor = {row, col};
      } 
    }
  }

  vector<pair<int, int>> dirs = {
    {1, 0},
    {0, 1},
    {-1, 0},
    {0, -1}
  }; 
  int dir = 0;
  int sum = 0;
  set<pair<int, int>> visited;

  while (true) {
    int i = cursor.first + dirs[dir].first;
    int j = cursor.second + dirs[dir].second;

    if (!visited.insert({i, j}).second) {
      sum++;
    }

    if (a[i][j] == '#') {
      dir = (dir + 1) % 4;
    } else {
      cursor = {i, j};
      visited.insert(cursor);
    }
  }

  cout << sum << endl;
}
