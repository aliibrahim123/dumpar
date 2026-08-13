// ali ibrahim 7358
// abdallah sammour 7215
// hassan ibrahim 7363 
#include<stdio.h>
void flush() {
	int c;
	while((c = getchar()) != '\n' && c != EOF);
}
int scan_int (char* prompt) {
	while (1) {
		printf("%s", prompt);
		int nb;
		if (scanf("%d", &nb) == 1) return nb;
		flush();
		printf("invalid input\n");
	}
}

int perrin (int n) {
	if (n == 0) return 3;
	if (n == 1) return 0;
	if (n == 2) return 2;

	return perrin(n - 2) + perrin(n - 3);
}
void ex1 () {
	int n = scan_int("enter a number: ");
	printf("the %d perrin number is %d\n", n, perrin(n));
}

int even (int arr[5], int ind) {
	if (ind == 5) return 0;
	int has_even = arr[ind] % 2 == 0;
	return has_even + even(arr, ind + 1);
}
void ex2 () {
	int arr[5];
	printf("enter 5 numbers\n");
	for (int i = 0; i < 5; i++) {
		arr[i] = scan_int("enter a number: ");
	}
	printf("the array has %d even numbers\n", even(arr, 0));
}

int sdiv_perfect (int k) {
	int sum = 1;
	for (int i = 2; i < k; i++) {
		if (k % i == 0) sum += i;
	}
	return sum;
}
void ex3 () {
	int nb = scan_int("enter a number: ");
	if (sdiv_perfect(nb) == nb) printf("%d is a perfect number\n", nb);
	else printf("%d is not a perfect number\n", nb);
}


int main () {
	while (1) {
		int ex = scan_int("type 1, 2 or 3 to run the specific exercise: ");
		switch (ex) {
			case 1: ex1(); return 0;
			case 2: ex2(); return 0;
			case 3: ex3(); return 0;
			default: printf("incorect input\n");
		}
	}
}