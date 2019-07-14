#include <iostream>

using namespace ::std;

bool find_in_2d_matrix(int *matrix, int columns, int rows, int number) {
    bool found = false;
    if (matrix != nullptr && columns > 0 && rows > 0) {
        int row = 0;
        int column = columns - 1;
        while (row < rows && column > 0) {
            if (matrix[row * columns + column] == number) {
                found = true;
                break;
            } else if (matrix[row * columns + column] < number) {
                row++;
            } else if (matrix[row * columns + column] > number) {
                column--;
            }
        }
    }
    return found;
}

int main() {
    int a[][4] = {{1, 2, 8,  9},
                  {2, 4, 9,  12},
                  {4, 7, 10, 13},
                  {6, 8, 11, 15}};
    if (!find_in_2d_matrix(*a, 4, 4, 11)) {
        cout << "Not found" << endl;
        return -1;
    }
    return 0;
}