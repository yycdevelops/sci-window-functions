#include <iostream>
#include <vector>
#include <math.h>
#include <cmath>

template<typename T> 
std::vector<T> hanning_window(size_t num) {
    std::vector<T> values;
    
    for (int i = 0; (i < num); i++) {
        T value = 0.5 - 0.5 * cos((M_PI * 2 * i / (num - 1)));
        values.push_back(value); 
    }
    return values;
}

template<typename T> 
std::vector<T> hamming_window(size_t num) {
    std::vector<T> values; 
    for (int i = 0; (i < num); i++) {
        T value = 0.54 - 0.46 * cos(M_PI * 2 * i / (num - 1));
        values.push_back(value);
    }
    return values;
}