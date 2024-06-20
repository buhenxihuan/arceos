#include "defs.h"

void cpy4(void *dst, const void *src, uint32_t size) {
    const uint32_t * ptr_src = src;
    uint32_p ptr_dst = dst;
    for (int i = 0; i < size; i += 4) {
        *ptr_dst = *ptr_src;
        ptr_src++;
        ptr_dst++;
    }
}


/*
 * load linux kernel image from <void *kernel_image> to <void *loc_real> (for real-mode part) and <void *loc_prot> (for protected-mode part) and fill kernel header
 * 
 * kernel_image: current physical addr of kernel image binary file, see qemu.mk.
 * loc_real: expected physical addr of read mode.
 * stack_end: end of boot stack
 * loc_prot: expected physical addr of protected mode.
 * initramfs
 * initramfs_size
 * 
 **/
int load_kernel(void *kernel_image, void *loc_real, void *loc_prot) {
    puts("[vlbl] loading kernel...");
    uint32_t kernel_lower_size = 0x5000;
    uint32_t kernel_upper_size = 0x2000000 - 0x100000;
    void *prot = kernel_image + 0x100000 - 0x1000;
    uint32_p ptr_dst = prot;
    for(int i = 0; i < kernel_upper_size; i += 4) {
        if(*ptr_dst == 0x12c007) {
            putux((uint32_t)(ptr_dst), true, 8);
        }
        ptr_dst++;
    }

    cpy4(loc_real, kernel_image, kernel_lower_size);
    cpy4(loc_prot, prot, kernel_upper_size);
    ptr_dst = (uint32_p)0x132000;
    if(*ptr_dst == 0x12c007) {
        puts("\nload OK!");
    }

    return 0;
}

int debug() {
    puts("debug...");
    return 0;
}
