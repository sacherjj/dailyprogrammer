#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define XY_MIN 0.0
#define XY_MAX 1.0
#define COMB_SIZE_MAX 3

typedef struct {
    double x;
    double y;
}
point_t;

typedef struct {
    point_t center;
    double radius;
}
circle_t;

int read_point(point_t *);
int point_in_square(point_t *);
void set_combs(unsigned long, unsigned long, unsigned long);
void set_circle_from_3_points(circle_t *, point_t *, point_t *, point_t *);
int same_points(point_t *, point_t *);
void set_circle_from_2_points(circle_t *, point_t *, point_t *);
void set_circle_from_2_segments(circle_t *, point_t *, point_t *, point_t *);
void set_center_from_2_segments(point_t *, double, double, double, double);
double segment_slope(point_t *, point_t *);
double segment_y_intercept(point_t *, point_t *);
int circle_in_square(circle_t *);
int valid_circle(circle_t *);
void set_circle(circle_t *, double, double, double);
void set_point(point_t *, double, double);
double euclidean_distance(point_t *, point_t *);
void print_circle(circle_t *);
void print_point(point_t *);

unsigned long points_n, points_half;
point_t *points, *comb[COMB_SIZE_MAX];
circle_t circle_min;

int main(void) {
unsigned long i;
point_t point_min;
    if (scanf("%lu", &points_n) != 1 || !points_n || points_n%2) {
        fprintf(stderr, "Invalid number of points\n");
        return EXIT_FAILURE;
    }
    points = malloc(sizeof(point_t)*points_n);
    if (!points) {
        fprintf(stderr, "Could not allocate memory for points\n");
        return EXIT_FAILURE;
    }
    for (i = 0; i < points_n && read_point(points+i); i++);
    if (i < points_n) {
        free(points);
        return EXIT_FAILURE;
    }
    points_half = points_n/2;
    set_point(&point_min, XY_MIN, XY_MIN);
    set_circle(&circle_min, XY_MIN, XY_MIN, XY_MAX-XY_MIN);
    if (points_half == 1) {
        set_combs(1UL, 0UL, 0UL);
    }
    else {
        set_combs(2UL, 0UL, 0UL);
        if (circle_min.radius == XY_MAX-XY_MIN) {
            set_combs(3UL, 0UL, 0UL);
        }
    }
    if (circle_min.radius < XY_MAX-XY_MIN) {
        print_circle(&circle_min);
    }
    else {
        puts("No solution");
    }
    free(points);
    return EXIT_SUCCESS;
}

int read_point(point_t *point) {
    if (scanf("%lf%lf", &point->x, &point->y) != 2 || !point_in_square(point)) {
        fprintf(stderr, "Invalid point\n");
        return 0;
    }
    return 1;
}

int point_in_square(point_t *point) {
    return point->x >= XY_MIN && point->y >= XY_MIN && point->x <= XY_MAX && point->y <= XY_MAX;
}

void set_combs(unsigned long comb_size, unsigned long comb_idx, unsigned long start) {
unsigned long i;
circle_t circle;
    if (comb_idx < comb_size) {
        for (i = start; i < points_n; i++) {
            comb[comb_idx] = points+i;
            set_combs(comb_size, comb_idx+1, i+1);
        }
    }
    else {
        if (comb_size == 3) {
            set_circle_from_3_points(&circle, comb[0], comb[1], comb[2]);
        }
        else if (comb_size == 2) {
            set_circle_from_2_points(&circle, comb[0], comb[1]);
        }
        else {
            set_circle(&circle, comb[0]->x, comb[0]->y, 0.0);
        }
        if (circle_in_square(&circle) && circle.radius < circle_min.radius && valid_circle(&circle)) {
            circle_min = circle;
        }
    }
}

void set_circle_from_3_points(circle_t *circle, point_t *point_a, point_t *point_b, point_t *point_c) {
    if (same_points(point_a, point_b)) {
        set_circle_from_2_points(circle, point_a, point_c);
    }
    else if (same_points(point_b, point_c)) {
        set_circle_from_2_points(circle, point_b, point_a);
    }
    else if (same_points(point_c, point_a)) {
        set_circle_from_2_points(circle, point_c, point_b);
    }
    else {
        if ((point_a->x == point_b->x && point_b->x == point_c->x) || (point_a->y == point_b->y && point_b->y == point_c->y)) {
            *circle = circle_min;
        }
        else {
            if (point_a->y == point_b->y) {
                set_circle_from_2_segments(circle, point_a, point_c, point_b);
            }
            else if (point_b->y == point_c->y) {
                set_circle_from_2_segments(circle, point_b, point_a, point_c);
            }
            else if (point_c->y == point_a->y) {
                set_circle_from_2_segments(circle, point_c, point_b, point_a);
            }
            else {
                set_circle_from_2_segments(circle, point_a, point_b, point_c);
            }
        }
    }
}

int same_points(point_t *point_a, point_t *point_b) {
    return point_a->x == point_b->x && point_a->y == point_b->y;
}

void set_circle_from_2_points(circle_t *circle, point_t *point_a, point_t *point_b) {
    set_point(&circle->center, (point_a->x+point_b->x)/2.0, (point_a->y+point_b->y)/2.0);
    circle->radius = euclidean_distance(point_a, point_b)/2.0;
}

void set_circle_from_2_segments(circle_t *circle, point_t *point_a, point_t *point_b, point_t *point_c) {
    set_center_from_2_segments(&circle->center, segment_slope(point_a, point_b), segment_y_intercept(point_a, point_b), segment_slope(point_b, point_c), segment_y_intercept(point_b, point_c));
    circle->radius = euclidean_distance(point_a, &circle->center);
}

void set_center_from_2_segments(point_t *center, double slope_ab, double y_intercept_ab, double slope_bc, double y_intercept_bc) {
    center->x = (y_intercept_ab-y_intercept_bc)/(slope_bc-slope_ab);
    center->y = slope_ab*center->x+y_intercept_ab;
}

double segment_slope(point_t *point_a, point_t *point_b) {
    return -(point_b->x-point_a->x)/(point_b->y-point_a->y);
}

double segment_y_intercept(point_t *point_a, point_t *point_b) {
    return (point_b->x*point_b->x-point_a->x*point_a->x+point_b->y*point_b->y-point_a->y*point_a->y)/(point_b->y-point_a->y)/2.0;
}

int circle_in_square(circle_t *circle) {
    return circle->center.x-circle->radius >= XY_MIN && circle->center.y-circle->radius >= XY_MIN && circle->center.x+circle->radius <= XY_MAX && circle->center.y+circle->radius <= XY_MAX;
}

int valid_circle(circle_t *circle) {
unsigned long points_count = 0, i;
    for (i = 0; i < points_n && points_count <= points_half; i++) {
        if (euclidean_distance(points+i, &circle->center) <= circle->radius) {
            points_count++;
        }
    }
    return points_count == points_half;
}

void set_circle(circle_t *circle, double x, double y, double radius) {
    set_point(&circle->center, x, y);
    circle->radius = radius;
}

void set_point(point_t *point, double x, double y) {
    point->x = x;
    point->y = y;
}

double euclidean_distance(point_t *point_a, point_t *point_b) {
double delta_x = point_a->x-point_b->x, delta_y = point_a->y-point_b->y;
    return sqrt(delta_x*delta_x+delta_y*delta_y);
}

void print_circle(circle_t *circle) {
    print_point(&circle->center);
    printf("%f\n", circle->radius);
}

void print_point(point_t *point) {
    printf("%f %f\n", point->x, point->y);
}
