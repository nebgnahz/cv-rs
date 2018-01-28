#ifndef CV_RS_COMMON_H
#define CV_RS_COMMON_H
typedef struct {
    int32_t x;
    int32_t y;
} Point2i;

typedef struct {
    float x;
    float y;
} Point2f;

typedef struct {
    int width;
    int height;
} Size2i;

typedef struct {
    float width;
    float height;
} Size2f;

typedef struct {
    int32_t x;
    int32_t y;
    int32_t width;
    int32_t height;
} Rect;

typedef struct {
    Point2f center;
    Size2f size;
    float angle;
} RotatedRect;

typedef struct {
    int32_t v0;
    int32_t v1;
    int32_t v2;
    int32_t v3;
} Scalar;

typedef struct {
    bool status;
    uint8_t* buf;
    size_t size;
} ImencodeResult;

// Caller is responsible for disposing `error` field
template<typename T>
struct Result
{
    T value;
    const char* error;

    static Result<T> FromFunction(std::function<T()> function)
    {
        Result<T> result = {};
        try
        {
            result.value = function();
        }
        catch( cv::Exception& e )
        {
            const char* err_msg = e.what();
            auto len = std::strlen(err_msg);
            auto retained_err = new char[len + 1];
            std::strcpy(retained_err, err_msg);
            result.error = retained_err;
        }
        return result;
    }
};

template<typename T>
struct CVec
{
    T* array;
    size_t size;
};
#endif //CV_RS_COMMON_H
