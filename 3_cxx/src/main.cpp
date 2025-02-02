#include <iostream>

extern "C"
{
    int add(int a, int b);
}

int main()
{
    int result = add(3, 4);
    std::cout << "Result of add(3, 4): " << result << std::endl;
    return 0;
}