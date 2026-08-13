// ali ibrahim 7358
// abdallah sammour 7215
// hassan ibrahim 7363 
#include<stdio.h>
#include<string.h>
void flush() {
	int c;
	while((c = getchar()) != '\n' && c != EOF);
}
int scan_int (char* prompt) {
	while (1) {
		printf("%s: ", prompt);
		int nb;
		int result = scanf("%d", &nb);
		flush();
		if (result == 1) return nb;
		printf("invalid input\n");
	}
}
int scan_bool (char* prompt) {
	while (1) {
		printf("%s (options: y, n): ", prompt);
		char choice;
		if (scanf(" %c", &choice) == 1) {
			flush();
			switch (choice) {
				case 'y': return 1;
				case 'n': return 0;
				default: {}
			}
		};
		flush();
		printf("invalid input\n");
	}
}

int findchar (char* str, char find) {
	for (int i = 0; str[i] != '\0'; i++) {
		if (str[i] == find) return i;
	}
	return -1;
}
void scan_str(char* prompt, char* str, int size) {
	printf("%s: ", prompt);
	fgets(str, size + 1, stdin);
	str[findchar(str, '\n')] = '\0';
}

char* super_marker_file = "super_market.dat";
typedef struct Item {
	char name[11];
	int nb;
	int price;
} Item;

void ex1_create_file (Item* items, int len) {
	FILE* fp = fopen(super_marker_file, "wb");

	if (fp == NULL) return;

	fwrite(&len, 4, 1, fp);
	for (int i = 0; i < len; i++)
		fwrite(&items[i], sizeof(Item), 1, fp);
	fclose(fp);
}

Item ex1_read_item(FILE* fp) {
	Item item;
	fread(&item, sizeof(Item), 1, fp);
	return item;
}

void ex1_has_item () {
	FILE* fp = fopen(super_marker_file, "rb");
	if (fp == NULL) return;
	int len;
	fread(&len, 4, 1, fp);

	int nb = scan_int("enter item nb");

	for (int i = 0; i < len; i++) {
		Item item = ex1_read_item(fp);
		if (item.nb == nb) {
			printf("found item: name = \"%s\", id = %x, price = %d\n", item.name, item.nb, item.price);
			fclose(fp);
			return;
		}
	}
	printf("item not found\n");
	fclose(fp);
}

void ex1_list_item () {
	FILE* fp = fopen(super_marker_file, "rb");
	if (fp == NULL) return;
	int len;
	fread(&len, 4, 1, fp);

	printf("item count: %d\n", len);
	for (int i = 0; i < len; i++) {
		Item item;
		fread(&item, sizeof(Item), 1, fp);
		printf("item %d: name = \"%-10s\" id = %x, price = %d\n", i + 1, item.name, item.nb, item.price);
	}
	fclose(fp);
}

void ex1_remove () {
	FILE* fp = fopen(super_marker_file, "rb");
	if (fp == NULL) return;
	int nb = scan_int("enter item nb");
	Item items[50] = {};
	int len;
	fread(&len, 4, 1, fp);

	for (int i = 0; i < len; i++) {
		Item item = ex1_read_item(fp);
		if (item.nb == nb) {
			printf("removed item: name = \"%s\", id = %x, price = %d\n", item.name, item.nb, item.price);
			for (int j = i; j < len; j++) items[j] = ex1_read_item(fp);
			fclose(fp);
			ex1_create_file(items, len -1);
			ex1_list_item();
			return;
		}
		items[i] = item;
	}
	printf("item not found\n");
	fclose(fp);
}

void ex1 () {
	int item_count = 5;
	Item items[50] = {
		(Item) {
			.name = "orange",
			.nb = 0x0001,
			.price = 1000,
		},
		(Item) {
			.name = "limon",
			.nb = 0x0002,
			.price = 2000,
		},
		(Item) {
			.name = "bumala",
			.nb = 0x0003,
			.price = 3000,
		},
		(Item) {
			.name = "abo_sora",
			.nb = 0x0004,
			.price = 1500,
		},
		(Item) {
			.name = "kalamantin",
			.nb = 0x0005,
			.price = 2500,
		},
		
	};

	if (!scan_bool("use default items")) {
		while (1) {
			int count = scan_int("enter item count (max 50)");
			if (count >= 0 && count <= 50) {
				item_count = count;
				break;
			}
		}
		for (int i = 0; i < item_count; i++) {
			printf("enter item (%d / %d)\n", i + 1, item_count);
			scan_str("enter item name", items[i].name, 10);
			items[i].nb = scan_int("enter item nb");
			items[i].price = scan_int("enter item price");
			printf("\n");
		}
	}

	ex1_create_file(items, item_count);

	printf("available commands: has, list, remove, exit\n");
	while (1) {
		char command[10];
		scan_str("enter command", command, 10);
		if (strcmp(command, "exit") == 0) return;
		else if (strcmp(command, "list") == 0) ex1_list_item();
		else if (strcmp(command, "has") == 0) ex1_has_item();
		else if (strcmp(command, "remove") == 0) ex1_remove();
		else printf("unkown command");
		printf("\n");
	}
}

typedef struct Author {
	int code;
	char name[51];
} Author;

typedef struct Book {
	int code;
	char title[51];
	int author;
	char subject[51];
	int copies;
} Book;

char* authors_file = "authors.bin";
char* books_file = "books.bin";

void ex2_create_authors (Author* authors, int len) {
	FILE* fp = fopen(authors_file, "wb");
	if (fp == NULL) return;

	fwrite(&len, 4, 1, fp);
	for (int i = 0; i < len; i++)
		fwrite(&authors[i], sizeof(Author), 1, fp);
	fclose(fp);
}
void ex2_create_books (Book* books, int len) {
	FILE* fp = fopen(books_file, "wb");
	if (fp == NULL) return;

	fwrite(&len, 4, 1, fp);
	for (int i = 0; i < len; i++)
		fwrite(&books[i], sizeof(Book), 1, fp);
	fclose(fp);
}

void ex2_list_authors () {
	FILE* fp = fopen(authors_file, "rb");
	if (fp == NULL) return;
	int len;
	fread(&len, 4, 1, fp);
	for (int i = 0; i < len; i++) {
		Author author;
		fread(&author, sizeof(Author), 1, fp);
		printf("author %d: name = \"%-20s\", code = %d\n", i + 1, author.name, author.code);
	}
	fclose(fp);
}

void ex2_list_books () {
	FILE* fp = fopen(books_file, "rb");
	if (fp == NULL) return;
	int len;
	fread(&len, 4, 1, fp);
	for (int i = 0; i < len; i++) {
		Book book;
		fread(&book, sizeof(Book), 1, fp);
		printf(
			"book %d: title = \"%-50s\", author = %d, subject = \"%-20s\", copies = %d\n", 
			i + 1, book.title, book.author, book.subject, book.copies
		);
	}
	fclose(fp);
}

void ex2_author () {
	int code = scan_int("enter author code");
	FILE* fp = fopen(authors_file, "rb");
	if (fp == NULL) return;
	int len;
	fread(&len, 4, 1, fp);
	for (int i = 0; i < len; i++) {
		Author author;
		fread(&author, sizeof(Author), 1, fp);
		if (author.code == code) {
			printf("found author: name = \"%s\", code = %d\n", author.name, author.code);
			fclose(fp);
			return;
		}
	}
	printf("author not found\n");
	fclose(fp);
}

void ex2_book () {
	int code = scan_int("enter book code");
	FILE* fp = fopen(books_file, "rb");
	if (fp == NULL) return;
	int len;
	fread(&len, 4, 1, fp);
	for (int i = 0; i < len; i++) {
		Book book;
		fread(&book, sizeof(Book), 1, fp);
		if (book.code == code) {
			printf(
				"found book: title = \"%s\", code = %d, author = %d, subject = \"%s\", copies = %d\n", 
				book.title, book.code, book.author, book.subject, book.copies
			);
			fclose(fp);
			return;
		}
	}
	printf("book not found\n");
	fclose(fp);
}

Book ex2_read_book(FILE* fp) {
	Book book;
	fread(&book, sizeof(Book), 1, fp);
	return book;
}

void ex2_add_book () {
	FILE* fp = fopen(books_file, "rb");
	if (fp == NULL) return;
	int len;
	fread(&len, 4, 1, fp);

	Book books[50];
	for (int i = 0; i < len; i++) books[i] = ex2_read_book(fp);
	fclose(fp);

	int code = scan_int("enter book code");
	char title[51];
	scan_str("enter book title", title, 51);
	int author = scan_int("enter author code");
	char subject[51];
	scan_str("enter book subject", subject, 51);
	int copies = scan_int("enter book copies");
	books[len] = (Book) {
		.code = code,
		.title = {},
		.author = author,
		.subject = {},
		.copies = copies
	};
	memcpy(books[len].title, title, 51);
	memcpy(books[len].subject, subject, 51);
	
	ex2_create_books(books, len + 1);
}

void ex2_borrow () {
	FILE* fp = fopen(books_file, "rb");
	if (fp == NULL) return;
	int len;
	fread(&len, 4, 1, fp);

	Book books[50];
	for (int i = 0; i < len; i++) books[i] = ex2_read_book(fp);
	fclose(fp);

	int code = scan_int("enter book code");
	for (int i = 0; i < len; i++) {
		if (books[i].code == code) {
			if (books[i].copies == 0) {
				printf("out of stock book: title = \"%s\", code = %d\n", books[i].title, books[i].code);
				return;
			}
			books[i].copies--;
			ex2_create_books(books, len);
			printf(
				"borrowed book: title = \"%s\", code = %d, copies = %d\n", 
				books[i].title, books[i].code, books[i].copies
			);
			return;
		}
	}
	printf("book not found\n");
} 

void ex2_restock () {
	FILE* fp = fopen(books_file, "rb");
	if (fp == NULL) return;
	int len;
	fread(&len, 4, 1, fp);

	Book books[50];
	for (int i = 0; i < len; i++) books[i] = ex2_read_book(fp);
	fclose(fp);

	int code = scan_int("enter book code");
	for (int i = 0; i < len; i++) {
		if (books[i].code == code) {
			int copies = scan_int("enter new copies");
			books[i].copies += copies;
			ex2_create_books(books, len);
			printf(
				"restocked book: title = \"%s\", code = %d, copies = %d\n", 
				books[i].title, books[i].code, books[i].copies
			);
			return;
		}
	}
	printf("book not found\n");
} 

void ex2 () {
	int author_count = 3, book_count = 5;

	Author authors[10] = {
		(Author) {
			.code = 7358,
			.name = "ali ibrahim"
		},
		(Author) {
			.code = 7215,
			.name = "abdallah sammour"
		},
		(Author) {
			.code = 7363,
			.name = "hassan ibrahim"
		}
	};
	Book books[50] = {
		(Book) {
			.code = 1,
			.title = "unimap: the ultimate guide",
			.author = 7358,
			.subject = "computer science",
			.copies = 5
		},
		(Book) {
			.code = 2,
			.title = "how to plant an orange tree",
			.author = 7215,
			.subject = "agriculture",
			.copies = 3
		},
		(Book) {
			.code = 3,
			.title = "how to plant a lemon tree",
			.author = 7215,
			.subject = "agriculture",
			.copies = 4
		},
		(Book) {
			.code = 4,
			.title = "neoview: a safe reactivity",
			.author = 7358,
			.subject = "computer science",
			.copies = 2
		},
		(Book) {
			.code = 5,
			.title = "what you dont know about the universe",
			.author = 7363,
			.subject = "physics",
			.copies = 1
		}
	};

	if (!scan_bool("use default items")) {
		while (1) {
			int count = scan_int("enter author count (max 10)");
			if (count >= 0 && count <= 10) {
				author_count = count;
				break;
			}
		}
		for (int i = 0; i < author_count; i++) {
			printf("enter author (%d / %d)\n", i + 1, author_count);
			authors[i].code = scan_int("enter author nb");
			scan_str("enter author name", authors[i].name, 10);
			printf("\n");
		}

		while (1) {
			int count = scan_int("enter book count (max 50)");
			if (count >= 0 && count <= 50) {
				book_count = count;
				break;
			}
		}
		for (int i = 0; i < book_count; i++) {
			printf("enter book (%d / %d)\n", i + 1, book_count);
			books[i].code = scan_int("enter book nb");
			scan_str("enter book title", books[i].title, 10);
			books[i].author = scan_int("enter author nb");
			scan_str("enter book subject", books[i].subject, 10);
			books[i].copies = scan_int("enter number of copies");
			printf("\n");
		}
	}

	ex2_create_authors(authors, author_count);
	ex2_create_books(books, book_count);

	printf("available commands: list authors, list books, author, book, add book, borrow, restock exit\n");
	while (1) {
		char command[15];
		scan_str("enter command", command, 15);
		if (strcmp(command, "exit") == 0) return;
		else if (strcmp(command, "list authors") == 0) ex2_list_authors();
		else if (strcmp(command, "list books") == 0) ex2_list_books();
		else if (strcmp(command, "author") == 0) ex2_author();
		else if (strcmp(command, "book") == 0) ex2_book();
		else if (strcmp(command, "add book") == 0) ex2_add_book();
		else if (strcmp(command, "borrow") == 0) ex2_borrow();
		else if (strcmp(command, "restock") == 0) ex2_restock();
		else printf("unkown command");
		printf("\n");
	}
}

int main () {
	while (1) {
		int ex = scan_int("type 1 or 2 to run the specific exercise: ");
		switch (ex) {
			case 1: ex1(); return 0;
			case 2: ex2(); return 0;
			default: printf("incorect input\n");
		}
	}
}