Enjoy the disaster with:
```
cargo objdump --lib --release -- --disassemble
```

Output I get:
```
Disassembly of section .text.prot_udiv:
00000000 <prot_udiv>:
       0: f3ef 8210     mrs     r2, primask
       4: b672          cpsid i                                               ; ...it should be after this...
       6: b129          cbz     r1, 0x14 <prot_udiv+0x14> @ imm = #0xa
       8: 07d2          lsls    r2, r2, #0x1f
       a: d100          bne     0xe <prot_udiv+0xe>     @ imm = #0x0
       c: b662          cpsie i                                               ; ...and before this.
       e: fbb0 f0f1     udiv    r0, r0, r1                                    ; <- this is our closure content...
      12: 4770          bx      lr
      14: b580          push    {r7, lr}
      16: 466f          mov     r7, sp
      18: f240 0000     movw    r0, #0x0
      1c: f2c0 0000     movt    r0, #0x0
      20: f7ff fffe     bl      0x20 <prot_udiv+0x20>   @ imm = #-0x4
```
