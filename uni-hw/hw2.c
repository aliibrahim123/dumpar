// ali ibrahim 7358
// abdallah sammour 7215
// hassan ibrahim 7363 
#include<stdio.h>
#include<math.h>

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
float scan_float (char* prompt) {
	while (1) {
		printf("%s", prompt);
		float nb;
		if (scanf("%f", &nb) == 1) return nb;
		flush();
		printf("invalid input\n");
	}
}
char scan_char (char* prompt) {
	while (1) {
		printf("%s", prompt);
		char nb;
		if (scanf(" %c", &nb) == 1) return nb;
		flush();
		printf("invalid input\n");
	}
}
int is_perfect (int n) {
	if (n == 1) return 0;
	int sum = 1;
	for (int i = 2; i < n; i++) {
		if (n % i == 0) sum += i;
	}
	return sum == n;
}
void ex1() {
	int low = scan_int("enter lower bound: ");
	int up = scan_int("enter upper bound: ");
	printf("perfect numbers between %d and %d: ", low, up);
	int is_first = 1;
	for (int i = low; i <= up; i++) {
		if (is_perfect(i)) {
			if (!is_first) printf(", ");
			is_first = 0;
			printf("%d", i);
		}
	}
	printf("\n");
}

typedef struct {
	int id;
	char name[21];
	float average;
} Student;
int average_of (Student students[5], int id) {
	for (int i = 0; i < 5; i++) {
		if (students[i].id == id) return students[i].average;
	}
	return -1;
}
int findchar (char* str, char find) {
	for (int i = 0; str[i] != '\0'; i++) {
		if (str[i] == find) return i;
	}
	return -1;
}
void ex2() {
	Student students[5] = {
		{ .id = 101, .name = "ali ibrahim", .average = 98.5 },
		{ .id = 102, .name = "ahmad ali", .average = 81.3 },
		{ .id = 103, .name = "hassan ibrahim", .average = 76.5 },
		{ .id = 104, .name = "abdullah samour", .average = 56.1 },
		{ .id = 105, .name = "ali hassan", .average = 65 }
	};

	for (int i = 0; i < 5; i++) {
		printf("student %d info:\n", i + 1);
		printf("name: %s\n", students[i].name);
		printf("id: %d\n", students[i].id);
		printf("average: %.1f\n", students[i].average);
		printf("\n");
	}

	int custom_table = scan_int("want a custom table? (1 for yes, 0 for no): ");

	if (custom_table) for (int i = 0; i < 5; i++) {
		printf("enter student %d info:\n", i + 1);
		printf("enter student name: ");
		flush();
		fgets(students[i].name, 21, stdin);
		students[i].name[findchar(students[i].name, '\n')] = '\0';
		students[i].id = scan_int("enter student id: ");
		students[i].average = scan_float("enter student average: ");
	}

	int id = scan_int("enter student id to retrieve his average: ");
	float avg = average_of(students, id);
	if (avg == -1) printf("student with id %d not found\n", id);
	else printf("student with id %d has average %.1f\n", id, avg);
}

typedef struct {
	char name;
	float x, y;
} Point;
void read (Point pts[4]) {
	printf("enter the points info in format name(x,y) where\n  name: char, \n  x, y: float\n");
	for (int i = 0; i < 4; i++) {
		printf("enter point %d / 4: ", i + 1);
		char name;
		float x, y;
		flush();
		if (scanf("%c(%f,%f)", &name, &x, &y) == 3) {
			pts[i] = (Point){ .name = name, .x = x, .y = y };
		} else {
			printf("invalid point\n");
			i--;
		}
	}
}
float distance (Point a, Point b) {
	float dx = a.x - b.x, dy = a.y - b.y;
	return sqrt(dx * dx + dy * dy);
}
Point middle (Point a, Point b) {
	return (Point) {
		.name = 'm',
		.x = (a.x + b.x) / 2,
		.y = (a.y + b.y) / 2
	};
}
Point find_point (Point pts[4], char name) {
	for (int i = 0; i < 4; i++) {
		if (pts[i].name == name) return pts[i];
	}
	return (Point){ .name = '\0' };
}
Point ask_point (Point pts[4], char* prompt) {
	while (1) {
		char name = scan_char(prompt);
		Point pt = find_point(pts, name);
		if (pt.name != '\0') return pt;
		printf("point with name %c not found\n", name);
	}
}
void ex3() {
	Point pts[4];
	read(pts);

	printf("inputed points:\n");
	for (int i = 0; i < 4; i++) {
		printf("pt %i: %c(%.2f, %.2f)\n", i + 1, pts[i].name, pts[i].x, pts[i].y);
	}

	printf("enter 2 points to calculate the distance between them\n");
	Point from = ask_point(pts, "from (by name): ");
	Point to = ask_point(pts, "to (by name): ");
	float dist = distance(from, to);
	printf("distance between %c and %c is %.2f\n", from.name, to.name, dist);

	printf("enter 2 points to calculate the midpoint between them\n");
	from = ask_point(pts, "from (by name): ");
	to = ask_point(pts, "to (by name): ");
	Point mid = middle(from, to);
	printf("midpoint between %c and %c is %c(%.2f, %.2f)\n", from.name, to.name, mid.name, mid.x, mid.y);
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