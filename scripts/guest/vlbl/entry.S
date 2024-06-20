
.section .text
.code16
.global entry
entry:
    cli
    cld

    # 初始化段寄存器
    xor     ax, ax
    mov     ds, ax
    mov     es, ax
    mov     ss, ax

    # 进入保护模式
    lgdt    [prot_gdt_desc]
    mov     eax, cr0
    or      eax, 0x1
    mov     cr0, eax

    ljmp    0x8, entry32

.code32
.global entry32
entry32:
    # 初始化段选择子到数据段
	out     0x4,al
    mov     ax, 0x10
    mov     ds, ax
    mov     es, ax
    mov     ss, ax
    mov     fs, ax
    mov     gs, ax

    # 初始化临时栈
    mov     ebp, 0x7b00
    mov     esp, 0x7b00

    # 加载内核
    push    0x00100000 # 32-bit 加载地址
    push    0x00001000 # 16-bit 加载地址
    push    0x70200000 # 镜像地址
    call    load_kernel
    call    debug
    mov     ecx, 0x1000       # kernel entry
    mov     eax, 0x2BADB002     # multiboot magic
    mov     ebx, 0x9500              # multiboot information (unsupported)
    mov     edx, 0x511
    jmp     ecx

.global end_of_code
end_of_code:

# 32位GDT
.balign 16
.global prot_gdt
prot_gdt:
    .quad 0x0000000000000000    # 0x00: null
    .quad 0x00cf9b000000ffff    # 0x08: code segment (base=0, limit=0xfffff, type=32bit code exec/read, DPL=0, 4k)
    .quad 0x00cf93000000ffff    # 0x10: data segment (base=0, limit=0xfffff, type=32bit data read/write, DPL=0, 4k)

.global prot_gdt_desc
prot_gdt_desc:
    .short  prot_gdt_desc - prot_gdt - 1    # limit
    .long   prot_gdt                        # base

