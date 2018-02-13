# CROSS=aarch64-none-elf
CROSS=aarch64-none-elf
CC=$(CROSS)-gcc
CFLAGS=-Wall -O -ffreestanding
USB=/media/brendan/NEW\ VOLUME

BLINKY_OBJS=crt0.o blinky.o

all: blinky.bin
deploy: blinky.bin
	cp blinky.bin $(USB)/kernel8.img
	cp ../files/firmware/* $(USB)
	umount $(USB)

%.o: %.S
	$(CROSS)-gcc -c $< -o $@

%.o: %.c
	$(CROSS)-gcc -c $< -o $@

blinky.bin: blinky.elf
	$(CROSS)-objcopy -O binary $< $@

blinky.elf: $(BLINKY_OBJS) layout.ld
	$(CROSS)-ld $(BLINKY_OBJS) -o $@ -Tlayout.ld

clean:
	rm -f $(BLINKY_OBJS) *.bin *.elf
