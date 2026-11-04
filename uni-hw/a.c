#include <stdio.h>
#include <stdlib.h>

struct Size {
	int x, y;
};

void arr_destroy(float* arr) {
	if (arr != NULL) {
		free(arr);
	}
}

void arr_copy(float* src, struct Size src_size, float* dst, struct Size dst_size) {
	for (int j = 0; j < src_size.y; j++) {
		for (int i = 0; i < src_size.x; i++) {
			if (j < dst_size.x && i < dst_size.y) {
				dst[j + i * dst_size.x] = src[j * src_size.x + i];
			}
		}
	}
}

float* resize_arr(float* arr, struct Size src_size, struct Size dst_size) {
	float* new_arr = malloc(sizeof(float) * dst_size.x * dst_size.y);
	arr_copy(arr, src_size, new_arr, dst_size);
	arr_destroy(arr);
	return new_arr;
}

int main() {
	int x, y;
	printf("enter size rowxcol: ");
	scanf("%dx%d", &y, &x);
	struct Size size = {x, y};
	float* arr = malloc(sizeof(float) * x * y);
	for (int i = 0; i < x * y; i++) {
		arr[i] = i;
	}
	printf("the arr is: \n");
	for (int j = 0; j < y; j++) {
		for (int i = 0; i < x; i++) {
			printf("%f ", arr[i + j * x]);
		}
		printf("\n");
	}
	printf("enter new size rowxcol: ");
	scanf("%dx%d", &y, &x);
	struct Size new_size = {x, y};
	arr = resize_arr(arr, size, new_size);
	printf("the new arr is: \n");
	for (int j = 0; j < y; j++) {
		for (int i = 0; i < x; i++) {
			printf("%f ", arr[i + j * x]);
		}
		printf("\n");
	}
	arr_destroy(arr);
}