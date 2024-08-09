#include "Light.h"
#include "LightSm.h"
#include <assert.h>
#include <string.h>
#include <stdio.h>

// we use a macro here (instead of a helper function) so that we get easy assert messages with line numbers
#define ASSERT_LIGHT_COLOR(expected) assert(strcmp(Light_get_color(), expected) == 0)


// prototypes
void test_inc_events(void);
void test_dim_events(void);
void test_off_events(void);


int main(void) {
    test_inc_events();
    test_dim_events();
    test_off_events();

    printf("all tests passed!!!\n");
}


void test_inc_events(void)
{
    printf("test inc events\n");

    LightSm sm;
    LightSm_ctor(&sm);
    LightSm_start(&sm);

    ASSERT_LIGHT_COLOR("off");

    LightSm_dispatch_event(&sm, LightSm_EventId_INC);
    ASSERT_LIGHT_COLOR("blue");

    LightSm_dispatch_event(&sm, LightSm_EventId_INC);
    ASSERT_LIGHT_COLOR("yellow");
    assert(sm.vars.count == 0);

    // count starts going up
    LightSm_dispatch_event(&sm, LightSm_EventId_INC);
    ASSERT_LIGHT_COLOR("yellow");
    assert(sm.vars.count == 1);

    LightSm_dispatch_event(&sm, LightSm_EventId_INC);
    ASSERT_LIGHT_COLOR("yellow");
    assert(sm.vars.count == 2);

    // count reaches 3, so we go back to off
    LightSm_dispatch_event(&sm, LightSm_EventId_INC);
    ASSERT_LIGHT_COLOR("off");
    assert(sm.vars.count == 3);
}


void test_dim_events(void)
{
    printf("test dim events\n");

    LightSm sm;
    LightSm_ctor(&sm);
    LightSm_start(&sm);

    ASSERT_LIGHT_COLOR("off");

    LightSm_dispatch_event(&sm, LightSm_EventId_INC);
    ASSERT_LIGHT_COLOR("blue");

    LightSm_dispatch_event(&sm, LightSm_EventId_DIM);
    ASSERT_LIGHT_COLOR("off");
}


void test_off_events(void)
{
    printf("test off events\n");

    LightSm sm;
    LightSm_ctor(&sm);
    LightSm_start(&sm);

    LightSm_dispatch_event(&sm, LightSm_EventId_INC);
    ASSERT_LIGHT_COLOR("blue");

    LightSm_dispatch_event(&sm, LightSm_EventId_OFF);
    ASSERT_LIGHT_COLOR("off");
}
