#include <iostream>
#include <vector>

using namespace std;

const int N = 10;

//时间复杂度O(n)，空间复杂度O(n)
//思路：新建一个大小为N的数组，记录每个值（下标）出现的次数，然后进行统计。
bool find_duplicated_numbers_1(vector<int> a, int *res)
{
    // 进行一些检查
    if (a.size() == 0)
    {
        return false;
    }
    for (int i = 0; i < a.size(); i++)
    {
        if (a[i] < 0 || a[i] > a.size() - 1)
        {
            return false;
        }
    }
    //定义一个向量，用来记录是否出现
    vector<int> b(N, 0);
    for (vector<int>::iterator iter = a.begin(); iter != a.end(); iter++)
    {
        b[*iter]++;
    }
    //将重复的加入返回值
    for (int i = 0; i < b.size(); i++)
    {
        if (b[i] > 1)
        {
            *res = i;
            return true;
        }
    }
    return false;
}

//时间复杂度O(n)，空间复杂度O(1)
//思路：由于数字大小最大不超过数组长度，则可以重排数字
//无重复的话，i应该在a[i]位置上，否则有重复
bool find_duplicated_numbers_2(vector<int> a, int *res)
{
    // 进行一些检查
    if (a.size() == 0)
    {
        return false;
    }
    for (int i = 0; i < a.size(); i++)
    {
        if (a[i] < 0 || a[i] > a.size() - 1)
        {
            return false;
        }
    }
    //查找重复数
    for (int i = 0; i < a.size(); i++)
    {
        while (a[i] != i)
        {
            if (a[i] == a[a[i]])
            {
                *res = a[i];
                return true;
            }

            int temp = a[i];
            a[i] = a[temp];
            a[temp] = temp;
        }
    }
    return false;
}

int main()
{
    int a[N] = {1, 3, 4, 6, 7, 2, 3, 6, 7, 0};
    vector<int> test(a, a + N);
    int res;
    //bool ok = find_duplicated_numbers_1(test, &res);
    bool ok = find_duplicated_numbers_2(test, &res);
    if (!ok)
    {
        cout << "Some thing wrong" << endl;
    }
    cout << res << endl;
}