#include "kernel.h"
#include "tty/tty.h"

#include "libc/stdio.h"

void kernel_main(const unsigned long magic,
	const void *boot_info, void *idt)
{
	tty_init();

	if(magic != MULTIBOOT2_BOOTLOADER_MAGIC)
	{
		panic("Non Multiboot2-compliant bootloader!");
	}

	if(((uintptr_t) boot_info) & 7)
	{
		panic("Boot informations structure's address is not aligned!");
	}

	printf("Booting crumbleos kernel version %s...\n", KERNEL_VERSION);
	printf("Retrieving Multiboot2 data...\n");

	const boot_info_t boot_info = read_boot_tags(boot_info);

	printf("Command line: %s\n", boot_info.cmdline);
	printf("Bootloader name: %s\n", boot_info.loader_name);
	printf("Memory lower bound: %u KB\n", boot_info.mem_lower);
	printf("Memory upper bound: %u KB\n", boot_info.mem_upper);

	memory_end = (void *) (boot_info.mem_upper * 1024);

	if(memory_end <= KERNEL_HEAP_BEGIN + KERNEL_HEAP_SIZE)
	{
		panic("Not enough heap space for kernel!");
	}

	printf("Available memory: %p bytes\n", memory_end);
	printf("Kernel memory manager initialization...\n");

	mm_init();

	// TODO
}

__attribute((noreturn))
void panic(const char *reason)
{
	tty_init();
	printf("--- KERNEL PANIC ---\n\nKernel has been forced to halt due to internal problem, sorry :/\nReason: %s\n\nIf you belive this is a bug on the kernel side, please feel free to report it.", reason);

	kernel_halt();
}
