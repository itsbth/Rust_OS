  global long_mode_start

  section .text
  bits 64
long_mode_start:
  call setup_SSE
  extern rust_main
  call rust_main
  mov rax, 0x4f724f204f534f4f
  mov [0xb8000], rax
  mov rax, 0x4f724f754f744f65
  mov [0xb8008], rax
  mov rax, 0x4f214f644f654f6e
  mov [0xb8010], rax
  hlt

error:
  mov rbx, 0x4f4f4f524f524f45
  mov [0xb8000], rbx
  mov rbx, 0x4f204f204f3a4f52
  mov [0xb8008], rbx
  mov byte [0xb800e], al
  hlt
  jmp error

setup_SSE:
  mov rax, 0x1
  cpuid
  test edx, 1 << 25
  jz .no_SSE

  mov rax, cr0
  and ax, 0xFFFB
  or ax, 0x2
  mov cr0, rax
  mov rax, cr4
  or ax, 3 << 9
  mov cr4, rax

  ret
  .no_SSE:
  mov al, "a"
  jmp error
