#include <iostream>
#include <vector>
#include <math.h>

using namespace std; 

std::vector<float> hanning_window(size_t num) {
    std::vector<float> values(num);

    for(auto t: values) {
        float value = 0.5 - 0.5 * (M_PI * 2 * t / (num - 1));
        values.push_back(value);
    }

    return values;
}