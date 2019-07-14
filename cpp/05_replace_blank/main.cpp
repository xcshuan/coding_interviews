#include <iostream>
#include <string>

using namespace ::std;

bool replace_blank(char *str, int length) {
    if (str == nullptr || length <= 0) {
        return false;
    }

    int original_length = 0;
    int new_length = 0;
    int i = 0;
    while (str[i] != '\0') {
        original_length++;
        new_length++;
        if (str[i] == ' ') {
            new_length += 2;
        }
        i++;
    }
    if (new_length + 1 > length) {
        return false;
    };
    str[new_length + 1] = '\0';
    int index_of_original = original_length;
    int index_of_new = new_length;
    while (index_of_original >= 0 && index_of_new > index_of_original) {
        if (str[index_of_original] == ' ') {
            str[index_of_new--] = '0';
            str[index_of_new--] = '2';
            str[index_of_new--] = '%';
        } else {
            str[index_of_new--] = str[index_of_original];
        }
        index_of_original--;
    }
    return true;
}

int main() {
    char a[100] = "www.google.com/a b c d";
    cout << a  << endl;
    bool ok = replace_blank(a, 100);
    if (!ok) {
        cout << "error" << endl;
        return -1;
    }
    cout << a << std::endl;
    return 0;
}