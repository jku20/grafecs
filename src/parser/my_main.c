/*========== my_main.c ==========

  This is the only file you need to modify in order
  to get a working mdl project (for now).

  my_main.c will serve as the interpreter for mdl.
  When an mdl script goes through a lexer and parser,
  the resulting operations will be in the array op[].

  Your job is to go through each entry in op and perform
  the required action from the list below:

  push: push a new origin matrix onto the origin stack

  pop: remove the top matrix on the origin stack

  move/scale/rotate: create a transformation matrix
                     based on the provided values, then
                     multiply the current top of the
                     origins stack by it.

  box/sphere/torus: create a solid object based on the
                    provided values. Store that in a
                    temporary matrix, multiply it by the
                    current top of the origins stack, then
                    call draw_polygons.

  line: create a line based on the provided values. Store
        that in a temporary matrix, multiply it by the
        current top of the origins stack, then call draw_lines.

  save: call save_extension with the provided filename

  display: view the screen
  =========================*/

#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>
#include <errno.h>
#include <stdint.h>
#include "parser.h"
#include "symtab.h"
#include "y.tab.h"

#include "matrix.h"
#include "ml6.h"
#include "display.h"
#include "draw.h"
#include "stack.h"
#include "gmath.h"

void my_main() {
    /*
    int i;
    struct matrix *tmp;
    struct stack *systems;
    screen t;
    zbuffer zb;
    color g;
    double step_3d = 20;
    double theta;

    //Lighting values here for easy access
    color ambient;
    ambient.red = 50;
    ambient.green = 50;
    ambient.blue = 50;

    double light[2][3];
    light[LOCATION][0] = 0.5;
    light[LOCATION][1] = 0.75;
    light[LOCATION][2] = 1;

    light[COLOR][RED] = 0;
    light[COLOR][GREEN] = 255;
    light[COLOR][BLUE] = 255;

    double view[3];
    view[0] = 0;
    view[1] = 0;
    view[2] = 1;

    //default reflective constants if none are set in script file
    struct constants white;
    white.r[AMBIENT_R] = 0.1;
    white.g[AMBIENT_R] = 0.1;
    white.b[AMBIENT_R] = 0.1;

    white.r[DIFFUSE_R] = 0.5;
    white.g[DIFFUSE_R] = 0.5;
    white.b[DIFFUSE_R] = 0.5;

    white.r[SPECULAR_R] = 0.5;
    white.g[SPECULAR_R] = 0.5;
    white.b[SPECULAR_R] = 0.5;

    //constants are a pointer in symtab, using one here for consistency
    struct constants *reflect;
    reflect = &white;

    systems = new_stack();
    tmp = new_matrix(4, 1000);
    clear_screen( t );
    clear_zbuffer(zb);
    g.red = 0;
    g.green = 0;
    g.blue = 0;

    print_symtab();
    */

    FILE *out = fopen("a.mdl_intermediate_language", "w+");
    if (out == NULL) {
        perror("fopen");
        return;
    }

    //the order of the constants is
    //'Ka_r', 'Ka_g', 'Ka_b', 'Kd_r', 'Kd_g', 'Kd_b', 'Ks_r', 'Ks_g', 'Ks_b'
    const double default_ks[9] = {0.1, 0.1, 0.1, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5};

    const uint8_t push = 0x1; 
    const uint8_t pop = 0x2; 
    const uint8_t move = 0x3; 
    const uint8_t rotate = 0x4;
    const uint8_t scale = 0x5;
    const uint8_t box = 0x6;
    const uint8_t sphere = 0x7;
    const uint8_t torus = 0x8;
    const uint8_t line = 0x9;
    const uint8_t save = 0xA;
    const uint8_t display = 0xB;
    //write mdl_intermediate_language based off the spec file
    //this should be parsed in my graphics engine, probably with binread
    for (int i=0;i<lastop;i++) { switch (op[i].opcode) {
            case PUSH:
                fwrite(&push, 1, 1, out);
                break;
            case POP:
                fwrite(&pop, 1, 1, out);
                break;
            case MOVE:
                fwrite(&move, 1, 1, out);
                fwrite(op[i].op.move.d, 8, 3, out);
                break;
            case ROTATE:
                fwrite(&rotate, 1, 1, out);
                fwrite(&op[i].op.rotate.axis, 1, 1, out);
                fwrite(&op[i].op.rotate.degrees, 1, 1, out);
                break;
            case SCALE:
                fwrite(&scale, 1, 1, out);
                fwrite(op[i].op.scale.d, 8, 1, out);
                break;
            case BOX:
                fwrite(&box, 1, 1, out);
                fwrite(op[i].op.box.d0, 8, 3, out);
                fwrite(op[i].op.box.d1, 8, 3, out);
                if (op[i].op.box.constants != NULL) {
                    fwrite(&op[i].op.box.constants->s.c->r[0], 8, 1, out);
                    fwrite(&op[i].op.box.constants->s.c->g[0], 8, 1, out);
                    fwrite(&op[i].op.box.constants->s.c->b[0], 8, 1, out);
                    fwrite(&op[i].op.box.constants->s.c->r[1], 8, 1, out);
                    fwrite(&op[i].op.box.constants->s.c->g[1], 8, 1, out);
                    fwrite(&op[i].op.box.constants->s.c->b[1], 8, 1, out);
                    fwrite(&op[i].op.box.constants->s.c->r[2], 8, 1, out);
                    fwrite(&op[i].op.box.constants->s.c->g[2], 8, 1, out);
                    fwrite(&op[i].op.box.constants->s.c->b[2], 8, 1, out);
                } else {
                    fwrite(default_ks, 8, 9, out);
                }
                break;
            case SPHERE:
                fwrite(&sphere, 1, 1, out);
                fwrite(op[i].op.sphere.d, 8, 3, out);
                fwrite(&op[i].op.sphere.r, 8, 1, out);
                if (op[i].op.sphere.constants != NULL) {
                    fwrite(&op[i].op.sphere.constants->s.c->r[0], 8, 1, out);
                    fwrite(&op[i].op.sphere.constants->s.c->g[0], 8, 1, out);
                    fwrite(&op[i].op.sphere.constants->s.c->b[0], 8, 1, out);
                    fwrite(&op[i].op.sphere.constants->s.c->r[1], 8, 1, out);
                    fwrite(&op[i].op.sphere.constants->s.c->g[1], 8, 1, out);
                    fwrite(&op[i].op.sphere.constants->s.c->b[1], 8, 1, out);
                    fwrite(&op[i].op.sphere.constants->s.c->r[2], 8, 1, out);
                    fwrite(&op[i].op.sphere.constants->s.c->g[2], 8, 1, out);
                    fwrite(&op[i].op.sphere.constants->s.c->b[2], 8, 1, out);
                } else {
                    fwrite(default_ks, 8, 9, out);
                }
                break;
            case TORUS:
                fwrite(&torus, 1, 1, out);
                fwrite(&op[i].op.torus.d, 8, 3, out);
                fwrite(&op[i].op.torus.r0, 8, 1, out);
                fwrite(&op[i].op.torus.r1, 8, 1, out);
                if (op[i].op.torus.constants != NULL) {
                    fwrite(&op[i].op.torus.constants->s.c->r[0], 8, 1, out);
                    fwrite(&op[i].op.torus.constants->s.c->g[0], 8, 1, out);
                    fwrite(&op[i].op.torus.constants->s.c->b[0], 8, 1, out);
                    fwrite(&op[i].op.torus.constants->s.c->r[1], 8, 1, out);
                    fwrite(&op[i].op.torus.constants->s.c->g[1], 8, 1, out);
                    fwrite(&op[i].op.torus.constants->s.c->b[1], 8, 1, out);
                    fwrite(&op[i].op.torus.constants->s.c->r[2], 8, 1, out);
                    fwrite(&op[i].op.torus.constants->s.c->g[2], 8, 1, out);
                    fwrite(&op[i].op.torus.constants->s.c->b[2], 8, 1, out);
                } else {
                    fwrite(default_ks, 8, 9, out);
                }
                break;
            case LINE:
                fwrite(&line, 1, 1, out);
                fwrite(&op[i].op.line.p0, 8, 3, out);
                fwrite(&op[i].op.line.p1, 8, 3, out);
                break;
            case SAVE:
                fwrite(&save, 1, 1, out);
                fwrite(&op[i].op.save.p->name, 1, strlen(op[i].op.save.p->name) + 1, out);
                break;
            case DISPLAY:
                fwrite(&display, 1, 1, out);
                break;
            case CONSTANTS:
                break;
            default:
                fprintf(stderr, "op code %d: %d not recognized\n", i, op[i].opcode);
                break;
        }
    }
}
