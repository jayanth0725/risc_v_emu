void print_string(const char* str);
void mmio_print_char(char c);
void exit_program(int code);

/* _start is the very first function in the file
so the compiler places it at memory address 0x0. */
void _start() {
    print_string("Hello from ecall!\n");

    mmio_print_char('M');
    mmio_print_char('M');
    mmio_print_char('I');
    mmio_print_char('O');
    mmio_print_char('\n');

    exit_program(0);
}

/* Using ecall (Syscall 64) for printing to the host terminal. */
void print_string(const char* str) {
    int len =0;
    while(str[len] != '\0')
        len++;

    /* Use inline RISC-V assembly to trigger a system call.
    Load the Syscall ID (64 for write) into register a7, the file 
    descriptor (1 for stdout) into a0, and the string details into a1/a2. */
    asm volatile (
        "li a7, 64\n"
        "li a0, 1 \n"
        "mv a1, %0\n"
        "mv a2, %1\n"
        "ecall\n"
        :
        : "r" (str), "r" (len)
        : "a0", "a1", "a2", "a7"
    );
}

/* Using the MMIO (Fake Screen) */
void mmio_print_char(char c) {
    volatile char* screen = (volatile char*)0x10000000;
    *screen = c;
}

/* Using ecall (Syscall 93) to exit cleanly. */
void exit_program(int code) {
    asm volatile (
        "li a7, 93\n"
        "mv a0, %0\n"
        "ecall\n"
        :
        : "r" (code)
        : "a0", "a7"
    );
}