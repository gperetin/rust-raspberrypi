/* This is the linker script.
 * From https://doc.ecoscentric.com/gnutools/doc/ld/Scripts.html#Scripts
 *
 * The main purpose of the linker script is to describe how the sections
 * in the input files should be mapped into the output file, and to control
 * the memory layout of the output file.
 *
 * The output file and each input file are in a special data format known
 * as an object file format. Each file is called an object file.
 *
 * Each object file has, among other things, a list of sections.
 * Each section in an object file has a name and a size. Most sections also
 * have an associated block of data, known as the section contents. A section
 * may be marked as loadable, which means that the contents should be loaded
 * into memory when the output file is run. A section with no contents may be
 * allocatable, which means that an area in memory should be set aside, but
 * nothing in particular should be loaded there (in some cases this memory
 * must be zeroed out).
 *
 * Every loadable or allocatable output section has two addresses. The first
 * is the VMA, or virtual memory address. This is the address the section will
 * have when the output file is run. The second is the LMA, or load memory
 * address. This is the address at which the section will be loaded. In most
 * cases the two addresses will be the same.
 *
 * Every object file also has a list of symbols, known as the symbol table.
 * A symbol may be defined or undefined. Each symbol has a name, and each
 * defined symbol has an address, among other information.
 *
 *  Simple program consists only of code, initialized data, and uninitialized
 *  data. These are in the ‘.text’, ‘.data’, and ‘.bss’ sections, respectively.
 */

SECTIONS
{
    /* Set current address to the value from which the RPi starts execution */
    /* The special linker variable dot `.' always contains the current output
     * location counter. Since the . always refers to a location in an output
     * section, it must always appear in an expression within a SECTIONS command.
     * Assigning a value to the . symbol will cause the location counter to be
     * moved. This may be used to create holes in the output section. The
     * location counter may never be moved backwards.
     */
    . = 0x80000;

    .text :
    /* Within curly braces, list the names of the input sections which should
     * be placed in this output section. '*' matches any name.
     * *(.text) means all '.text' input sections in all input files.
     */
    {
        *(.text._start) *(.text*)
    }

    /*
     * The special output section name ‘/DISCARD/’ may be used to discard input
     * sections. Any input sections which are assigned to an output section named
     * ‘/DISCARD/’ are not included in the output file.
     */
    /DISCARD/ : { *(.comment*) }
}
