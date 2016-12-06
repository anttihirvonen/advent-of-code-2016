#include <fstream>
#include <iostream>
#include <algorithm>
#include <iterator>
#include <cstring>

using namespace std;

int main()
{
    std::fstream file("input-06.txt");
    unsigned letter_freqs[8][26];
    memset(letter_freqs, 0xFFFFFFFF, sizeof(letter_freqs));

    for (std::string line; getline(file, line);)
    {
        for (int i = 0; i < line.length(); ++i)
        {
            auto index = line[i] - 'a';
            letter_freqs[i][index] -= 1 << 8;
            letter_freqs[i][index] &= (0xFFFFFF00 | line[i]);
        }
    }

    for (auto& freqs : letter_freqs)
    {
        std::sort(std::begin(freqs), std::end(freqs));
        std::cout << static_cast<char>(freqs[25] & 0xFF);
    }

    return 0;
}
