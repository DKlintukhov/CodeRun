/*1. Средний элемент
Решена
Лёгкая

Рассмотрим три числа a a, b b и c c. Упорядочим их по возрастанию.

Какое число будет стоять между двумя другими?

Решение этой задачи на С++ могло бы выглядеть так:

#include <iostream>
#include <algorithm>

using namespace std;

int main()
{
    int a[3];
    for (int i = 0; i < 3; ++i) cin >> a[i];
    sort(a, a + 3);
    cout << a[1] << endl;
    return 0;
}

Формат ввода

В единственной строке записаны три целых числа a a, b b, c c ( − 1000 ≤ a , b , c ≤ 1000 −1000≤a,b,c≤1000), числа разделены одиночными пробелами.
Формат вывода

Выведите число, которое будет стоять между двумя другими после упорядочивания.
*/

#include <bits/stdc++.h>

int main(int argc, const char** argv)
{
    constexpr size_t SIZE = 3;
    
    int arr[SIZE];
    for (size_t i = 0; i < SIZE; ++i)
        std::cin >> arr[i];

    std::sort(std::begin(arr), std::end(arr));

    std::cout << arr[SIZE / 2] << std::endl;

    return 0;
}

