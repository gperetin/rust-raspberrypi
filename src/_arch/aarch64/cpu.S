.section ".text._start"

// .global exports the given symbol (_start in this case), so ld can see it
// _start is the default entry-point for a program, that's what the ld looks for
// by default, so we use that.
// Docs: https://docs.huihoo.com/redhat/rhel-4-docs/rhel-as-en-4/global.html
.global _start

_start:
1: wfe          // wait for event
   b        1b  // In case event happened, jump to 1
