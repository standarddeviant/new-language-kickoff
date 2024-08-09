#include "Light.h"

const char* light_color = "?";

void Light_off() {
    light_color = "off";
}

void Light_blue() {
    light_color = "blue";
}

void Light_yellow() {
    light_color = "yellow";
}

const char* Light_get_color() {
    return light_color;
}
