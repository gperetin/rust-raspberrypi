.section ".text._start"

.global _start

_start:
1: wfe          // wait for event
   b        1b  // In case event happened, jump to 1
