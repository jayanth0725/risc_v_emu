# Makefile

CC = riscv64-linux-gnu-gcc
LD = riscv64-linux-gnu-ld
OBJCOPY = riscv64-linux-gnu-objcopy

CFLAGS = -march=rv32i -mabi=ilp32 -ffreestanding -nostdlib -c
LDFLAGS = -m elf32lriscv -N -Ttext 0x0

all: test.bin

test.bin: main.elf
	$(OBJCOPY) -O binary main.elf test.bin

main.elf: main.o
	$(LD) $(LDFLAGS) main.o -o main.elf

main.o: main.c
	$(CC) $(CFLAGS) main.c -o main.o

clean:
	rm -f *.o *.elf test.bin



