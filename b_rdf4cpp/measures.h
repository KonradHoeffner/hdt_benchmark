#ifndef MEASURES
#define MEASURES

#include <cstdlib>
#include <cstdio>
#include <cstring>
#include <ctime>

ulong get_nanosec() {
  int status;
  struct timespec timespec{};
  unsigned long long ret = 0;
  status = clock_gettime(CLOCK_MONOTONIC, &timespec);
  if (status != 0) {
    perror("error in clock_gettime:");
    exit(1);
  }
  ret += static_cast<ulong>(timespec.tv_sec) * 1000000000UL + static_cast<ulong>(timespec.tv_nsec);
  return ret;
}

long parse_line(char *line) {
  // This assumes that a digit will be found and the line ends in " Kb".
  auto i = static_cast<long>(strlen(line));
  const char *p = line;
  while (*p < '0' || *p > '9') p++;
  line[i - 3] = '\0';
  i = static_cast<long>(atoi(p));
  return i;
}

long get_vmsize() { //Note: this value is in KB!
  FILE *file = fopen("/proc/self/status", "r");
  long result = -1;
  char line[128];

  while (fgets(line, 128, file) != nullptr) {
    if (strncmp(line, "VmRSS:", 6) == 0) {
      result = parse_line(line);
      break;
    }
  }
  fclose(file);
  return result;
}

#endif

