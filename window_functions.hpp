#include <iostream>
#include <vector>
#include <math.h>
#include <cmath>
#include <numeric> 
#include <algorithm>

// Private method 
template<typename T>
std::vector<T> fill_vec(size_t num) {
    std::vector<T> vec(num);
    std::iota(std::begin(vec), std::end(vec), 0);
    return vec;
}

template<typename T> 
std::vector<T> hanning_window(size_t num) {
    auto value_vector = fill_vec<T>(num);
    for (auto &i : value_vector) {
        i = 0.5 - 0.5 * cos((M_PI * 2 * i / (num - 1)));
    }
    return value_vector;
}

template<typename T> 
std::vector<T> hamming_window(size_t num) {
    std::vector<T> values = fill_vec<T>(num);
    for (auto &i : values) {
        i  = 0.54 - 0.46 * cos(M_PI * 2 * i / (num - 1));
    }
    return values;
}

template<typename T>
std::vector<T> bartlett_window(size_t num) {
    std::vector<T> values = fill_vec<T>(num);
    for (auto &i : values) {
        i = 1 - abs(2 * (i - 0.5*(num-1)) / (num-1));
    }
    return values; 
}

template<typename T>
std::vector<T> triangular_window(size_t num) {
    std::vector<T> values = fill_vec<T>(num);
    const int DEMON = num % 2 != 0 ? num + 1 : num;
    for (auto &i : values) {
        i = (1 - abs(2.0 * i - (num - 1)) / DEMON);
    }
    return values;
}

template<typename T>
std::vector<T> cosine_window(size_t num) {
    std::vector<T> values = fill_vec<T>(num);
    for (auto &i : values) {
        i = sin(M_PI * i / (num - 1));
    }
    return values;
}

// The rectangle window uses alogithm to fill with 1's. 
std::vector<float> rectangle_window(size_t num) {
    // Uses the standard library to fill 
    std::vector<float> values(num);
    std::fill(values.begin(), values.end(), 1.0);
    return values;
}

template<typename T> 
std::vector<T> nuttall_window(size_t num) {
    std::vector<T> values = fill_vec<T>(num);
    //data_point * PI * x as f32 / (num as f32 - 1.0)

    for (auto &i : values) {
        i = 0.355768 - 0.487396 * cos(2 * M_PI * i / (num -1))
         + 0.144232 * cos(4 * M_PI * i / (num - 1))
         - 0.012604 * cos(6 * M_PI * i / (num -1));
    }

    return values;
}