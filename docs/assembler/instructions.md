# Assembly Instructions

The assembly contains a small number of instructions. The assembly has a different (but similar) set of instructions than the CPU, which is listed in the [ISA document](../cpu/isa.md#instructions). 

Each assembly instruction may map to different CPU instructions. The different set is merely a convenience.

Each assembly instruction is enumerated in the following table:

<table>
<tr>
<th>Instruction</th>
<th>Description</th>
<th>CPU Instruction</th>
<th>Example</th>
<th>Pseudo Code</th>
</tr>

<tr>
<td>nop</td>
<td>No operation</td>
<td>NOP</td>
<td> 
      
`nop;` 
      
</td>
<td>

`;`
    
</td>
</tr>

<tr>
<!-- LD -->
<td rowspan="6">ld</td>
<td rowspan="6">Load</td>
<td>LD</td>
<td>
      
`ld r1, *123;`
      
</td>
<td>

`r1 = *123`
      
</td>
</tr>

<tr>
<td>LDR</td>
<td>

`ld r1, *(r2 + 3);`
      
</td>
<td>

`r1 = *(r2 + 3)`
    
</td>
</tr>
  
<tr>
<td>LDI</td>
<td>

`ld r1, 123;`
      
</td>
<td>

`r1 = 123`
      
</td>
</tr>
  
<tr>
<td>ST</td>
<td>

`ld *123, r1;`
      
</td>
<td>

`*123 = r1`
      
</td>
</tr>
  
<tr>
<td>STR</td>
<td>

`ld *(r1 + 3), r2;`
      
</td>
<td>

`*(r1 + 3) = r2`
      
</td>
</tr>
  
<tr>
<td>ALU.PASSA</td>
<td>

`ld r1, r2;`
      
</td>
<td>

`r1 = r2`
      
</td>
</tr>
<tr>
<td>push</td>
<td>Push to stack</td>
<td>PUSH</td>
<td>

`push r1;`
      
</td>
<td>

`*(--sp) = r1`
      
</td>
</tr>
<tr>
<td>pop</td>
<td>Pop from stack</td>
<td>POP</td>
<td>

`pop r1;`
      
</td>
<td>

`r1 = *(sp++)`
      
</td>
</tr>

<tr>
<td>int</td>
<td>Software interrupt</td>
<td>INT</td>
<td>

`int;`
      
</td>
<td>

`*(--sp) = pc` <br>
`*(--sp) = status` <br>
`status &= 0b111100` <br>
`pc = 2`
      
</td>
</tr>
  
<tr>
<td>and</td>
<td>Bitwise AND</td>
<td>ALU.AND</td>
<td>
      
`and r1, r2, r3;` <br>
`and r1, r2, 123;` <br>
`and r1, r2;` <br>
`and r1, 123;`
      
</td>
<td>

`r1 = r2 & r3` <br>
`r1 = r2 & 123` <br>
`r1 = r1 & r2` <br>
`r1 = r1 & 123`
      
</td>
</tr>
    
<tr>
<td>or</td>
<td>Bitwise OR</td>
<td>ALU.OR</td>
<td>
      
`or r1, r2, r3;` <br>
`or r1, r2, 123;` <br>
`or r1, r2;` <br>
`or r1, 123;`
      
</td>
<td>

`r1 = r2 | r3` <br>
`r1 = r2 | 123` <br>
`r1 = r1 | r2` <br>
`r1 = r1 | 123`
      
</td>
</tr>
  
<tr>
<td>xor</td>
<td>Bitwise exclusive OR</td>
<td>ALU.XOR</td>
<td>
      
`xor r1, r2, r3;` <br>
`xor r1, r2, 123;` <br>
`xor r1, r2;` <br>
`xor r1, 123;`
      
</td>
<td>

`r1 = r2 ^ r3` <br>
`r1 = r2 ^ 123` <br>
`r1 = r1 ^ r2` <br>
`r1 = r1 ^ 123`
      
</td>
</tr>
  
<tr>
<td>not</td>
<td>Bitwise NOT</td>
<td>ALU.NOT</td>
<td>
      
`not r1, r2;` <br>
`not r1, 123;` <br>
`not r1;` <br>
      
</td>
<td>

`r1 = ~r2` <br>
`r1 = ~123` <br>
`r1 = ~r1`
      
</td>
</tr>

<tr>
<td>add</td>
<td>Addition</td>
<td>ALU.ADD</td>
<td>
      
`add r1, r2, r3;` <br>
`add r1, r2, 123;` <br>
`add r1, r2;` <br>
`add r1, 123;`
      
</td>
<td>

`r1 = r2 + r3` <br>
`r1 = r2 + 123` <br>
`r1 = r1 + r2` <br>
`r1 = r1 + 123`
      
</td>
</tr>
  
<tr>
<td>sub</td>
<td>Subtraction</td>
<td>ALU.SUB</td>
<td>
      
`sub r1, r2, r3;` <br>
`sub r1, r2, 123;` <br>
`sub r1, 123, r2;` <br>
`sub r1, r2;` <br>
`sub r1, 123;` <br>
`sub 123, r1;`
      
</td>
<td>

`r1 = r2 - r3` <br>
`r1 = r2 - 123` <br>
`r1 = 123 - r2` <br>
`r1 = r1 - r2` <br>
`r1 = r1 - 123` <br>
`r1 = 123 - r1`

</td>
</tr>

<tr>
<td>neg</td>
<td>Binary negation</td>
<td>ALU.NEG</td>
<td>
      
`neg r1, r2;` <br>
`neg r1, 123;` <br>
`neg r1;` <br>
      
</td>
<td>

`r1 = -r2` <br>
`r1 = -123` <br>
`r1 = -r1`
      
</td>
</tr>

<tr>
<td>shl</td>
<td>Shift left</td>
<td>ALU.SHL</td>
<td>
      
`shl r1, r2, r3;` <br>
`shl r1, r2, 123;` <br>
`shl r1, r2;` <br>
`shl r1, 123;`
      
</td>
<td>

`r1 = r2 << r3` <br>
`r1 = r2 << 123` <br>
`r1 = r1 << r2` <br>
`r1 = r1 << 123`
      
</td>
</tr>
<tr>
<td>shr</td>
<td>Logical shift right</td>
<td>ALU.SHR</td>
<td>
      
`shr r1, r2, r3;` <br>
`shr r1, r2, 123;` <br>
`shr r1, r2;` <br>
`shr r1, 123;`
      
</td>
<td>

`r1 = r2 >> r3` <br>
`r1 = r2 >> 123` <br>
`r1 = r1 >> r2` <br>
`r1 = r1 >> 123`
      
</td>
</tr>
  
<tr>
<td>ashr</td>
<td>Arithmetic shift right</td>
<td>ALU.ASHR</td>
<td>
      
`ashr r1, r2, r3;` <br>
`ashr r1, r2, 123;` <br>
`ahsr r1, r2;` <br>
`ashr r1, 123;`
      
</td>
<td>

`r1 = r2 >>> r3` <br>
`r1 = r2 >>> 123` <br>
`r1 = r1 >>> r2` <br>
`r1 = r1 >>> 123`
      
</td>
</tr>

<tr>
<td>rol</td>
<td>Rotate left</td>
<td>ALU.ROL</td>
<td>
      
`rol r1, r2, r3;` <br>
`rol r1, r2, 123;` <br>
`rol r1, r2;` <br>
`rol r1, 123;`
      
</td>
<td>

`r1 = (r2 << r3 % 32) | (r2 >> (32 - r3) % 32)` <br>
`r1 = (r2 << 123 % 32) | (r2 >> (32 - 123) % 32)` <br>
`r1 = (r1 << r2 % 32) | (r1 >> (32 - r2) % 32)` <br>
`r1 = (r1 << 123 % 32) | (r1 >> (32 - 123) % 32)` <br>
      
</td>
</tr>

<tr>
<td>ror</td>
<td>Rotate right</td>
<td>ALU.ROR</td>
<td>
      
`ror r1, r2, r3;` <br>
`ror r1, r2, 123;` <br>
`ror r1, r2;` <br>
`ror r1, 123;`
      
</td>
<td>

`r1 = (r2 >> r3 % 32) | (r2 << (32 - r3) % 32)` <br>
`r1 = (r2 >> 123 % 32) | (r2 << (32 - 123) % 32)` <br>
`r1 = (r1 >> r2 % 32) | (r1 << (32 - r2) % 32)` <br>
`r1 = (r1 >> 123 % 32) | (r1 << (32 - 123) % 32)` <br>
      
</td>
</tr>

</table>

# Instruction Modifiers

The assembler allows adding modifiers to the instruction mnemonics. These modify the functionality of an instruction. Some modifiers work on all instructions, while others work on only a subset of the instructions.

## ALU Modifiers

These modifiers only work on ALU instructions (those listed ALU.* in the [Instructions section](#assembly-instructions)).

The ALU modifiers allow for conditionally updating the status register with the NZCV flags, as described in the [ISA document](../cpu/isa.md#status-register).

The below table enumerates each modifier:

| ALU Modifier | Description | Example | Pseudo Code |
|---|---|---|---|
| s | Sets the NZCV from the result of the ALU operation | `sub.s r1, r2;` | `r1 = r1 - r2` <br> `status.NZCV = <new flags>` |
| t | Sets the NZCV from the result of the ALU operation, but ignores the result | `and.t r1, r2;` | `r1 & r2` <br> `status.NZCV = <new flags>` |

## Condition Modifiers

These modifiers work on all instructions.

The condition modifiers allow for conditional execution, which makes the instruction only execute if the condition is met.

These conditions are mapped to CPU conditions as seen in the [ISA document](../cpu/isa.md#conditions).

The below table enumerates each condition:

<table>
<tr>
<th>Condition</th>
<th>Description</th>
<th>Example</th>
<th>CPU Condition</th>
<th>Pseudo Code</th>
</tr>

<tr>
<td></td>
<td>Always executes</td>
<td>

`ld pc, 123;`
      
</td>
<td>NONE</td>
<td>

`pc = 123`

</td>
</tr>

<tr>
<td>eq</td>
<td>
      
`sub.t x, y;` where `x == y`

<td>

`ld.eq pc, 123;`
      
</td>
<td rowspan="2">EQ</td>
<td rowspan="2">

`if (Z) { pc = 123 }`

</td>
</tr>

<tr>
<td>zs</td>
<td>
      
Zero flag is set; alias for `eq`

</td>
<td>

`ld.zs pc, 123;`
      
</td>
</tr>

<tr>
<td>ne</td>
<td>
      
`sub.t x, y;` where `x != y`

<td>

`ld.ne r1, r2;`
      
</td>
<td rowspan="2">NE</td>
<td rowspan="2">

`if (!Z) { r1 = r2 }`

</td>
</tr>

<tr>
<td>zc</td>
<td>
      
Zero flag is clear; alias for `ne`

</td>
<td>

`ld.zc r1, r2;`
      
</td>
</tr>

<tr>
<td>neg</td>
<td>
      
The last ALU operation (with S set) resulted in a negative number if interpreted as a signed value

<td>

`ld.neg *r1, r2;`
      
</td>
<td rowspan="2">NEG</td>
<td rowspan="2">

`if (N) { *r1 = r2 }`

</td>
</tr>

<tr>
<td>ns</td>
<td>
      
Negative flag is set; alias for `neg`

</td>
<td>

`ld.ns *r1, r2;`
      
</td>
</tr>

<tr>
<td>pos</td>
<td>
      
The last ALU operation (with S set) resulted in a positive number if interpreted as a signed value

<td>

`ld.pos *(fp + 3), r0;`
      
</td>
<td rowspan="2">POS</td>
<td rowspan="2">

`if (!N) { *(fp + 3) = r0 }`

</td>
</tr>

<tr>
<td>nc</td>
<td>
      
Negative flag is clear; alias for `pos`

</td>
<td>

`ld.nc *(fp + 3), r0;`
      
</td>
</tr>

<tr>
<td>vs</td>
<td>
      
The last ALU operation (with S set) resulted in a signed overflow

</td>
<td>

`ld.vs *(fp + 5), r3;`
      
</td>
<td>VS</td>
<td>

`if (V) { *(fp + 5) = r3 }`

</td>
</tr>

<tr>
<td>vc</td>
<td>
      
The last ALU operation (with S set) did not result in a signed overflow

</td>
<td>

`ld.vc pc, r5;`
      
</td>
<td>VC</td>
<td>

`if (!V) { pc = r5 }`

</td>
</tr>

<tr>
<td>ult</td>
<td>
      
`sub.t x, y;` where x and y are unsigned, and `x < y`

<td>

`sub.ult fp, 123;`
      
</td>
<td rowspan="2">ULT</td>
<td rowspan="2">

`if (!C) { fp = fp - 123 }`

</td>
</tr>

<tr>
<td>cc</td>
<td>
      
Carry flag is clear; alias for `ult`

</td>
<td>

`sub.cc fp, 123;`
      
</td>
</tr>

<tr>
<td>ugt</td>
<td>
      
`sub.t x, y;` where x and y are unsigned, and `x > y`

<td>

`add.ugt r1, r3, r5;`
      
</td>
<td>UGT</td>
<td>

`if (C && !Z) { r1 = r3 + r5 }`

</td>
</tr>

<tr>
<td>ule</td>
<td>
      
`sub.t x, y;` where x and y are unsigned, and `x <= y`

<td>

`sub.ule pc, r5;`
      
</td>
<td>ULE</td>
<td>

`if (!C || Z) { pc = pc - r5 }`

</td>
</tr>

<tr>
<td>uge</td>
<td>
      
`sub.t x, y;` where x and y are unsigned, and `x >= y`

<td>

`int.uge;`
      
</td>
<td rowspan="2">UGE</td>
<td rowspan="2">

`if (C) { ... }`

</td>
</tr>

<tr>
<td>cs</td>
<td>
      
Carry flag is set; alias for `uge`

</td>
<td>

`int.cs;`
      
</td>
</tr>

<tr>
<td>slt</td>
<td>
      
`sub.t x, y;` where x and y are signed, and `x < y`

<td>

`push.slt lr;`
      
</td>
<td>SLT</td>
<td>

`if (N !== V) { ... }`

</td>
</tr>

<tr>
<td>sgt</td>
<td>
      
`sub.t x, y;` where x and y are signed, and `x > y`

<td>

`pop.sgt r0;`
      
</td>
<td>SGT</td>
<td>

`if (!Z && (N == V)) { ... }`

</td>
</tr>

<tr>
<td>sle</td>
<td>

`sub.t x, y;` where x and y are signed, and `x <= y`

<td>

`shl.sle r1, r2, r3;`
      
</td>
<td>SLE</td>
<td>

`if (Z || (N != V)) { r1 = r2 << r3 }`

</td>
</tr>

<tr>
<td>sge</td>
<td>

`sub.t x, y;` where x and y are signed, and `x >= y`

<td>

`ashr.sge r1, 123;`
      
</td>
<td>SGE</td>
<td>

`if (N == V) { r1 = r1 >>> 123 }`

</td>
</tr>

</table>

> [!NOTE]
> Although some condition mnemonics are for subtraction, it still works for any ALU operation. For example, `and.t r1, 1;` and `ld.ne pc, 123;` will only jump if the least significant bit of `r1` is set to `1`. To make your code clearer, you may use the aliases if the subtraction mnemonics can cause confusion, for example, `ld.zc pc, 123;`.

# Operands

Instruction operands are simply expressions. Expressions are documented in the [Expressions document](expressions.md). 

Each instruction only allows for certain numbers of operands. For example, `add` only works with 2 or 3 operands. Additionally, all instructions will only work with specific types in each operand. Expression result types are detailed in the [Expressions document](expressions.md#result-types).

The examples in the [Assembly Instructions section](#assembly-instructions) outline all the possible variations for each instruction.
