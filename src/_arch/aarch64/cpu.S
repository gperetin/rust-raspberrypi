.section ".text._start"

// .global exports the given symbol (_start in this case), so ld can see it
// _start is the default entry-point for a program, that's what the ld looks for
// by default, so we use that.
// Docs: https://docs.huihoo.com/redhat/rhel-4-docs/rhel-as-en-4/global.html
.global _start

// Good manual with instructions and registers
// Programmer's Guide - https://static.docs.arm.com/den0024/a/DEN0024A_v8_architecture_PG.pdf
// ARM ARM - https://static.docs.arm.com/ddi0487/fb/DDI0487F_b_armv8_arm.pdf
_start:
    // MRS (Move System Register)
    // mpidr_el1 is Multi-Processor Affinity Register, and it allows us to
    // determine on which core we are running
    // For the format of mpidr_el1, check p.3349 of the ARM ARM
    mrs     x1, mpidr_el1

    // Clear all the bits except [1:0] which hold core ID
    // Format is: <destination register, input register, operand2>
    // operand2 can be a register or a constant (as in this case)
    and     x1, x1, #3

    // If this is core 0, jump to label 2
    // cbz is "Compare and Branch if zero"
    // Label has a 'f' suffix to tell the compiler we're jumping forward
    // Though, reading ARM ARM manual, page F1-4038, seems like cbz does
    // not even support backward branch
    cbz     x1, 2f

1:  wfe          // wait for event
    b       1b  // In case event happened, jump to 1

2: 
    // Load address of the function "_start" into x1
    ldr     x1, =_start

    // set start of stack to the first address before _start
    // TODO: I'm not sure why this sets it to the "first address before", where
    // was that address math calculated?
    mov     sp, x1

    // Jump to the runtime_init kernel function
    // bl stores the return address (which is the address of the instruction after
    // bl) in the register X30. bl also hints to the branch prediction logic that
    // this is a function call
    // Per ARM ARM, we can only use bl or blr to perform a nested subroutine call
    // when the subrouting is expected to return to the immediatelly following
    // instruction
    // bl stands for "Branch with Link"
    bl      runtime_init

    // This should never be reached, just in case, park this core too
    b       1b
