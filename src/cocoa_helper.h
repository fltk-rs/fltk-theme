#pragma once

#ifndef COCOA_COLOR_DEFINITION

#define COLOR_GET(color)                                                       \
  void my_##color(double *r, double *g, double *b, double *a);

#else

#define COLOR_GET(color)                                                       \
  void my_##color(double *r, double *g, double *b, double *a) {                \
    NSColor *i = [NSColor color];                                              \
    NSColor *c = [i colorUsingColorSpace:[NSColorSpace genericRGBColorSpace]]; \
    [c getRed:r green:g blue:b alpha:a];                                       \
  }

#endif

COLOR_GET(windowBackgroundColor)

COLOR_GET(labelColor)

COLOR_GET(controlBackgroundColor)


