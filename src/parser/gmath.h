#ifndef GMATH_H
#define GMATH_H

#include "matrix.h"
#include "ml6.h"
#include "symtab.h"

// constants for lighting
#define LOCATION 0
#define COLOR 1
#define RED 0
#define GREEN 1
#define BLUE 2


#define SPECULAR_EXP 4

// lighting functions
color get_lighting( double *normal, double *view, color alight, double light[2][3], struct constants *reflect);
color calculate_ambient(color alight, struct constants *reflect );
color calculate_diffuse(double light[2][3], struct constants *reflect, double *normal );
color calculate_specular(double light[2][3], struct constants *reflect, double *view, double *normal );
void limit_color( color * c );

// vector functions
void normalize( double *vector );
double dot_product( double *a, double *b );
double *calculate_normal(struct matrix *polygons, int i);

#endif
