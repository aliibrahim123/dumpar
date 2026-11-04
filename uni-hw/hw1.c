#include <math.h>
#include <stdio.h>

void flush() {
	int c;
	while ((c = getchar()) != '\n' && c != EOF)
		;
}

void ex1() {
	printf("enter a string (max 10): ");
	char a[11];
	fgets(a, 11, stdin);
	printf("enter a char to find: ");
	char c;
	scanf("%c", &c);

	for (int i = 0; a[i] != '\0'; i++) {
		if (a[i] == c) {
			printf("find char %c at pos %d\n", c, i);
			return;
		}
	}
	printf("didnt find char %c in input\n", c);
}

int cmpstr(char *a, char *b) {
	int ind = 0;
	while (1) {
		char cur_a = a[ind], cur_b = b[ind];
		if (cur_a > cur_b)
			return -1;
		if (cur_a < cur_b)
			return 1;
		if (cur_a == '\0' && cur_b == '\0')
			return 0;
		ind++;
	}
}
int findchar(char *str, char find) {
	for (int i = 0; str[i] != '\0'; i++) {
		if (str[i] == find)
			return i;
	}
	return -1;
}
void ex2() {
	char words[10][26];
	for (int i = 0; i < 10; i++) {
		printf("enter word %d / 10 (max 25): ", i + 1);
		fgets(words[i], 26, stdin);
		words[i][findchar(words[i], '\n')] = '\0';
		if (words[i][0] == '\0')
			i--;
	}

	int word_inds[10] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
	for (int i = 0; i < 10; i++) {
		for (int j = i; j < 10; j++) {
			if (cmpstr(words[word_inds[i]], words[word_inds[j]]) == -1) {
				int tmp = word_inds[j];
				word_inds[j] = word_inds[i];
				word_inds[i] = tmp;
			}
		}
	}

	printf("sorted: \n");
	for (int i = 0; i < 10; i++)
		printf("%d: %s\n", i + 1, words[word_inds[i]]);
}

typedef struct {
	char name;
	float x, y;
} Point;
void ex3() {
	Point pts[4];
	printf("the format of a point: name(x, y) where\n  name: char, \n  x, y: "
		   "float\n");
	for (int i = 0; i < 4; i++) {
		char name;
		float x, y;
		printf("enter point %d / 4: ", i + 1);
		if (scanf("%c(%f, %f)", &name, &x, &y) == 3) {
			pts[i] = (Point){.name = name, .x = x, .y = y};
		} else {
			printf("invalid point\n");
			i--;
		}
		flush();
	}

	printf("inputed points:\n");
	for (int i = 0; i < 4; i++)
		printf("pt %i: %c(%f, %f)\n", i + 1, pts[i].name, pts[i].x, pts[i].y);

	printf("select 2 points to calculate the distance\n");
	int from, to;
from_loop:
	while (1) {
		char name;
		printf("from (by name): ");
		scanf(" %c", &name);
		if (pts[0].name == name) {
			from = 0;
			break;
		}
		if (pts[1].name == name) {
			from = 1;
			break;
		}
		if (pts[2].name == name) {
			from = 2;
			break;
		}
		if (pts[3].name == name) {
			from = 3;
			break;
		}
		printf("not found\n");
	}
to_loop:
	while (1) {
		char name;
		printf("to (by name): ");
		scanf(" %c", &name);
		if (pts[0].name == name) {
			to = 0;
			break;
		}
		if (pts[1].name == name) {
			to = 1;
			break;
		}
		if (pts[2].name == name) {
			to = 2;
			break;
		}
		if (pts[3].name == name) {
			to = 3;
			break;
		}
		printf("not found\n");
	}

	float dx = pts[to].x - pts[from].x;
	float dy = pts[to].y - pts[from].y;
	float dst = sqrtf(dx * dx + dy * dy);
	printf("distance between %c and %c: %f\n", pts[from].name, pts[to].name, dst);
}

int main() {
	printf("type 1, 2 or 3 to run the specific exercise: ");
	int ex;
	scanf("%d", &ex);
	flush();
	switch (ex) {
	case 1:
		ex1();
		break;
	case 2:
		ex2();
		break;
	case 3:
		ex3();
		break;
	default:
		printf("incorect input\n");
	}
	return 0;
}