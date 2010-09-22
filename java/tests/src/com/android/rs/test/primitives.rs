#include "shared.rsh"

#pragma rs export_func(rs_primitives_test)

// Testing primitive types
#pragma rs export_var(floatTest)
#pragma rs export_var(doubleTest)
static float floatTest = 1.99f;
static double doubleTest = 2.05;
static char charTest = -8;
static short shortTest = -16;
static int intTest = -32;
static uchar ucharTest = 8;
static ushort ushortTest = 16;
static uint uintTest = 32;

static void test_primitive_types(uint32_t index) {
    bool failed = false;
    start();

    _RS_ASSERT(floatTest == 1.99f);
    _RS_ASSERT(doubleTest == 2.05);
    _RS_ASSERT(charTest == -8);
    _RS_ASSERT(shortTest == -16);
    _RS_ASSERT(intTest == -32);
    _RS_ASSERT(ucharTest == 8);
    _RS_ASSERT(ushortTest == 16);
    _RS_ASSERT(uintTest == 32);

    float time = end(index);
    if (failed) {
        rsDebug("test_primitives FAILED ", time);
    }
    else {
        rsDebug("test_primitives PASSED ", time);
    }
}


void rs_primitives_test(uint32_t index, int test_num) {
    test_primitive_types(index);
}

