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

template<typename T>
std::vector<T> bartlett_window(size_t num) {
    std::vector<T> values;
    for (int i = 0; (i < num); i++) {
        //T value = abs(1 - (2 * i - 0.5 * num - 1) / num - 1);
        T value = 1 - abs(2 * (i - 0.5*(num-1)) / (num-1));
        values.push_back(value);
    }
    return values; 
}

template<typename T>
std::vector<T> triangular_window(size_t num) {
    std::vector<T> values; 
    const int DEMON = num % 2 != 0 ? num + 1 : num;

    for (int i = 0; (i < num); i++) {
        T value = (1 - abs(2.0 * i - (num - 1)) / DEMON);
        values.push_back(value);
    }
    return values;
}

template<typename T>
std::vector<T> cosine_window(size_t num) {
    std::vector<T> values; 
    for (int i = 0; (i < num); i++) {
        T value = sin(M_PI * i / (num - 1));
        values.push_back(value);
    }

    return values;
}