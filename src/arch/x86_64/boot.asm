  global start
  extern long_mode_start

  section .text
  bits 32
start:
  mov esp, stack_top
  call test_multiboot
  call test_cpuid
  call test_long_mode
  call setup_page_tables
  call enable_paging

  lgdt [gdt64.pointer]
  mov ax, gdt64.data
  mov ss, ax
  mov ds, ax
  mov es, ax

  jmp gdt64.code:long_mode_start

  ;; Print OK
  mov dword [0xb8000], 0x2f4b2f4f
  hlt

error:
  mov dword [0xb8000], 0x4f524f45
  mov dword [0xb8004], 0x4f3a4f52
  mov dword [0xb8008], 0x4f204f20
  mov byte [0xb800a], al
  hlt

test_multiboot:
  cmp eax, 0x36d76289
  jne .no_multiboot
  ret
  .no_multiboot:
  mov al, "0"
  jmp error

test_cpuid:
  pushfd
  pop eax
  mov ecx, eax
  xor eax, 1 << 21
  push eax
  popfd
  pushfd
  pop eax
  push ecx
  popfd
  xor eax, ecx
  jz .no_cpuid
  ret
  .no_cpuid:
  mov al, "1"
  jmp error

test_long_mode:
  mov eax, 0x80000000
  cpuid
  cmp eax, 0x80000001
  jb .no_long_mode
  mov eax, 0x80000001
  cpuid
  test edx, 1 << 29
  jz .no_long_mode
  ret
  .no_long_mode:
  mov al, "2"
  jmp error

setup_page_tables:
  mov eax, p3_table
  or eax, 11b
  mov [p4_table], eax
  mov dword [p3_table], 10000011b
  ret

enable_paging:
  mov eax, p4_table
  mov cr3, eax

  mov eax, cr4
  or eax, 1 << 5
  mov cr4, eax

  mov ecx, 0xC0000080
  rdmsr
  or eax, 1 << 8
  wrmsr

  mov eax, cr0
  or eax, 1 << 31
  or eax, 1 << 16
  mov cr0, eax

  ret

  section .bss
  align 4096
p4_table:
  resb 4096
p3_table:
  resb 4096
stack_bottom:
  resb 64
stack_top:

  section .rodata
gdt64:
  dq 0
  .code: equ $ - gdt64
  dq (1<<44) | (1<<47) | (1<<41) | (1<<43) | (1<<53) ; code segment
  .data: equ $ - gdt64
  dq (1<<44) | (1<<47) | (1<<41) ; data segment
  .pointer:
  dw $ - gdt64 - 1
  dq gdt64