#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdio.h>
#include <string.h>
#include <ctype.h>

void print_hello() {
    printf("hello world!\n");
}

void print_help() {
    printf("help message:\n");
    printf("  cat <filename> - print file content\n");
    printf("  echo <message> - print message\n");
    printf("  help - print help message\n");
}

int cat_file(const char *filename) {
    char line[512];
    int fd, count, total;

    fd = open(filename, O_RDONLY);
    if (fd < 0) {
        return fd;
    }

    total = 0;
    while ((count = read(fd, line, sizeof(line) - 1)) > 0) {
        line[count] = '\0';
        total += count;
        printf("%s", line);
    }
    close(fd);
    if (count == 0) {
        return total;
    } else {
        return count;
    }
}

int start_with(const char *str, const char *prefix, const char **rest) {
    int prefix_len = strlen(prefix);

    if (strncmp(str, prefix, prefix_len) == 0) {
        if (rest != NULL) {
            *rest = str + prefix_len;

            while (**rest && isspace(**rest)) {
                (*rest)++;
            }
        }
        return 1;
    } else {
        return 0;
    }
}

int remove_trailing_newline(char *str) {
    int len = strlen(str);
    if (len > 0 && str[len - 1] == '\n') {
        str[len - 1] = '\0';
        return 1;
    } else {
        return 0;
    }
}

int main() {
    print_hello();
    print_help();

    char line[512];
    int line_count = 0;
    const char *rest;

    while (1) {
        printf("%04d > ", line_count);
        fflush(stdout);
        fgets(line, sizeof(line), stdin);
        remove_trailing_newline(line);

        if (start_with(line, "cat", &rest)) {
            if (rest == NULL || *rest == '\0') {
                printf("cat: missing filename\n");
            } else {
                int ret = cat_file(rest);
                if (ret < 0) {
                    printf("cat: failed to open file: %s\n", rest);
                } else {
                    printf("cat: read %d bytes\n", ret);
                }
            }
        } else if (start_with(line, "echo", &rest)) {
            if (rest == NULL || *rest == '\0') {
                printf("echo: missing message\n");
            } else {
                printf("%s\n", rest);
            }
        } else if (start_with(line, "help", &rest)) {
            print_help();
        } else {
            printf("unknown command: %s\n", line);
        }

        line_count += 1;
    }
    
    return 0;
}
