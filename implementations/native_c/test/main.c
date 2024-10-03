#include "../src/pycode_h5.h"
#define FILENAME "/home/leonardo/Documents/unige/data/18-07-2024/38927/raw/2024-07-18T15-32-4638927_100E_DIV70_nbasal_0001_E-00155.h5"

int main (int argc, char *argv[]) {
  pycodeh5_init();
  PhaseH5 phase;
  init_phase(&phase);
  phaseh5_error res = phase_open(&phase, FILENAME);
  if (res == OK) {
    printf("%s\n", phase.date);
  } else {
    printf("%s -> %d\n", "ERROR", res);
  }
  raw_data(&phase, 1, 0, 100, NULL);
  phase_close(&phase);
  pycodeh5_close();
  return 0;
}
