# Public Registers

There are 11, 32-bit, general purpose register (`R0` to `R10`)

There are 4 special purpose registers with varying widths. The table below enumerates all registers and their purpose.

| Register | Code | Description | Width |
|---|---|---|---|
| R0 | 0x0 | General purpose | 32 |
| R1 | 0x1 | General purpose | 32 |
| R2 | 0x2 | General purpose | 32 |
| R3 | 0x3 | General purpose | 32 |
| R4 | 0x4 | General purpose | 32 |
| R5 | 0x5 | General purpose | 32 |
| R6 | 0x6 | General purpose | 32 |
| R7 | 0x7 | General purpose | 32 |
| R8 | 0x8 | General purpose | 32 |
| R9 | 0x9 | General purpose | 32 |
| R10 | 0xa | General purpose | 32 |
| STATUS | 0xb | State of the CPU; conditions, interupt mask, and mode | 6 |
| SP | 0xc | Stack pointer; points to last item in stack, and grows down | 32 |
| LR | 0xd | Link register; is set to previous `PC` value if `PCLINK` is written to | 32 |
| PCLINK | 0xe | Program counter; A pseudo register used to load `PC` and load `LR` with the previous `PC` value (much like some ISA's jump with link) | 32 |
| PC | 0xf | Program counter; points to next instruction to run | 32 |

> [!NOTE]
> The assembler has more registers that will alias to this set, notably `fp` which aliases to `r10`. The extra aliased registers are documented in the [Expressions document](../assembler/expressions.md#register). Upercase distinguishes a register as seen by the CPU and registers as seen by the assembler.

## Status Register

Among all the special purpose registers, the status register is the only register with a different width.

The layout of the status register is as follows:

| 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|
| N | Z | C | V | I | M |

The status register is made of various flags. These flags are sumarized in the table below:

| Flag | Description |
|---|---|
| N | The last ALU operation with S=1 resulted in a negative number |
| Z | The last ALU operation with S=1 resulted in zero |
| C | The last ALU operation that with S=1 resulted in a carry |
| V | The last ALU operation that with S=1 resuted in a signed overflow |
| I | Hardware interrupt mask, i.e., I=0 means no hardware interrupts will occur |
| M | If set to 0, CPU is in supervisor mode, if set to 1, CPU is in user mode | 

The first four of these flags are used for conditional execution. The following table details the condition, the corresponding NZCV flags, and the condition code.

### Conditions

| Condition | Code | Description | NZCV Expression |
|---|---|---|---|
| NONE | 0x0 | always executes | `true` |
| EQ | 0x1 | `sub.t x, y` where `x == y` | `Z` |
| NE | 0x2 | `sub.t x, y` where `x != y` | `!Z` |
| NEG | 0x3 | Last ALU operation resulted in negative number if interpreted as a signed value | `N` |
| POS | 0x4 | Last ALU operation resulted in positive number if interpreted as a signed value | `!N` |
| Vs | 0x5 | Last ALU operation resulted in a signed overflow | `V` |
| Vc | 0x6 | Last ALU operation did not result in a signed overflow | `!V` |
| ULT | 0x7 | `sub.t x, y` where x and y are unsigned, and `x < y` | `!C` |
| UGT | 0x8 | `sub.t x, y` where x and y are unsigned, and `x > y`  | `C && !Z` |
| ULE | 0x9 | `sub.t x, y` where x and y are unsigned, and `x <= y` | `!C \|\| Z` |
| UGE | 0xa | `sub.t x, y` where x and y are unsigned, and `x >= y`  | `C` |
| SLT | 0xb | `sub.t x, y` where x and y are signed, and `x < y`  | `N !== V` |
| SGT | 0xc | `sub.t x, y` where x and y are signed, and `x > y`  | `!Z && (N == V)` |
| SLE | 0xd | `sub.t x, y` where x and y are signed, and `x <= y`  | `Z \|\| (N != V)` |
| SGE | 0xe | `sub.t x, y` where x and y are signed, and `x >= y`  | `N == V` |

> [!NOTE]
> The assembler has more conditions that will alias to this set. The extra aliased conditions are documented in the [Instructions document](../assembler/instructions.md#condition-modifiers). Upercase distinguishes conditions as seen by the CPU and conditions as seen by the assembler.

# Instruction

There is only 10 instruction from the CPU's perspective. The ALU instruction is generic and works with all supported ALU operations. The instructions are enumerated in the following table:

| Instruction | Code | Description | Pseudo Code |
|---|---|---|---|
| NOP | 0x00 | No operation | `;` |
| LD | 0x01 | Load | `register = *address` |
| LDR | 0x02 | Load from register | `register = *(register + offset)` |
| LDI | 0x03 | Load immediate | `register = immediate` |
| ST | 0x04 | Store | `*address = register` |
| STR | 0x05 | Store to register | `*(register + offset) = register` |
| PUSH | 0x06 | Push to stack | `*(--sp) = register` |
| POP | 0x07 | Pop from stack | `register = *(sp++)` |
| INT | 0x08 | Software interrupt; see [Interupt Vector Table section](#interrupt-vector-table) | `*(--sp) = pc` <br> `status &= 0b111110` <br> `pc = 2` |
| ALU | 0xf_ | Performs an ALU operation as shown in the [ALU Operations section](#alu-operations) | `A = B <op> C` <br> `if (S) { status.NZCV = <new flags> }` |

> [!NOTE]
> LD and ST offset is a signed value. so it is possible to do solmething like `ld *(r1 - 123), r2`.

> [!NOTE]
> The assembler has a different set of instructions that will mapped to these instructions. E.g. `ld r1, 123` gets mapped to a LDI instuction. Uppercase distinguishes the instruction as a CPU instruction rather than an assembly instruction. Assembly instructions are documented in the [Instructions document](../assembler/instructions.md#assembly-instructions).

## Layout

Each instruction is 32 bits; there is no variable length instructions.

The ALU operations can be found in the [ALU Operations section](#alu-operations).

The condition codes can be found in the [Conditions section](#conditions).

The registers codes can be found in the [Public Reigsters section](#public-registers).

<table>

<tr>
<th>Op</th>
<th>31</th>
<th>30</th>
<th>29</th>
<th>28</th>
<th>27</th>
<th>26</th>
<th>25</th>
<th>24</th>
<th>23</th>
<th>22</th>
<th>21</th>
<th>20</th>
<th>19</th>
<th>18</th>
<th>17</th>
<th>16</th>
<th>15</th>
<th>14</th>
<th>13</th>
<th>12</th>
<th>11</th>
<th>10</th>
<th>9</th>
<th>8</th>
<th>7</th>
<th>6</th>
<th>5</th>
<th>4</th>
<th>3</th>
<th>2</th>
<th>1</th>
<th>0</th>
</tr>

<tr>
<td>NOP</td>
<td colspan="4" rowspan="11">Condition Code</td>
<td colspan="8">Instruction Code = 0x00</td>
<td colspan="24" rowspan="2">Unused</td>
</tr>

<tr>
<td>INT</td>
<td colspan="8">Instruction Code = 0x08</td>
</tr>

<tr>
<td>LD</td>
<td colspan="8">Instruction Code = 0x01</td>
<td colspan="4" rowspan="2">Register A</td>
<td colspan="16" rowspan="2">Address</td>
</tr>

<tr>
<td>ST</td>
<td colspan="8">Instruction Code = 0x04</td>
</tr>

<tr>
<td>LDR</td>
<td colspan="8">Instruction Code = 0x02</td>
<td colspan="4" rowspan="2">Register A</td>
<td colspan="4" rowspan="2">Register B</td>
<td colspan="12" rowspan="2">Offset</td>
</tr>

<tr>
<td>STR</td>
<td colspan="8">Instruction Code = 0x05</td>
</tr>

<tr>
<td>LDI</td>
<td colspan="8">Instruction Code = 0x03</td>
<td colspan="4">Register A</td>
<td colspan="16">Immediate</td>
</tr>

<tr>
<td>PUSH</td>
<td colspan="8">Instruction Code = 0x06</td>
<td colspan="4" rowspan="2">Register A</td>
<td colspan="16" rowspan="2">Unused</td>
</tr>

<tr>
<td>POP</td>
<td colspan="8">Instruction Code = 0x07</td>
</tr>

<tr>
<td rowspan="2">ALU</td>
<td colspan="4" rowspan="2">Instruction Code High = 0xf</td>
<td colspan="4" rowspan="2">ALU Instruction  Code</td>
<td>I=0</td>
<td rowspan="2">R</td>
<td rowspan="2">Ln</td>
<td rowspan="2">S</td>
<td colspan="4" rowspan="2">Register A</td>
<td colspan="4" rowspan="2">Register B</td>
<td colspan="4">Register C</td>
<td colspan="4">Unused</td>
</tr>

<tr>
<!-- ALU with Immediate -->
<td>I=1</td>
<td colspan="8">Immediate</td>
</tr>

</table> 

# ALU Operations

The CPU Operation "ALU" shown above allows passing in an "ALU Instruction Code." The ALU operations and their corresponding instruction code is shown below.

> [!NOTE]
> "Register A", "Register B", and "Register C" correspond to A, B, and C in the table below, although, if I=1 (immediate bit it set), then C = immediate, and if R=1 (reverse bit is set), then "Register B" corresponds to C, and "Register C" (or an immediate) corresponds to B (i.e., reversed), and if Ln=1 (Loadn bit is set), then A is not set to any register.

| Op | Code | Pseudo Code |
|---|---|---|
| PASS | 0x0 | `A = C` |
| AND | 0x1 | `A = B & C` |
| OR | 0x2 | `A = B \| C` |
| XOR | 0x3 | `A = B ^ C` |
| NOT | 0x4 | `A = ~C` |
| ADD | 0x5 | `A = B + C` |
| ADDC | 0x6 | `A = B + C + carry` |
| SUB | 0x7 | `A = B - C` |
| SUBB | 0x8 | `A = B - C - borrow` |
| NEG | 0x9 | `A = -C` |
| SHL | 0xa | `A = B << C` |
| SHR | 0xb | `A = B >> C` |
| ASHR | 0xc | `A = B >>> C` |

> [!NOTE]
> Borrow is simply ~carry

## ALU Flags

You may have noticed the ALU CPU instruction contains four bits named "I", "R", "Ln", and "S". These flags modify the ALU's behaviour. The following table sumarizes these behaviours:

| Flag | Description | Purpose |
|---|---|---|
| I | Immediate flag; if set to 1, then the last 16 bits of the instruction is interpreted as an immediate value | So we can do stuff like `add, R1, 123` |
| R | Reverse flag; if set to 1, then the role of Register B, and Register C (or immediate) is flipped | So we can do stuff like `sub 123, R1`; not really useful for if I=0 |
| Ln | Loadn flag; if set to 1, then do not load the result of the operation into Register A | So we can do stuff like `sub.t, r1, r2`; not really useful for if S=0, but note the .t is like `TST` is other ISAs |
| S | Set status flag; if set to 1, then sets the status register with it's various flags | So we can do stuff like `sub.s, r1, r2` |

# Interrupt Vector Table

The addresses of various locations in memory the CPU may jump to in each case are as follows:

| Purpose | Address | When |
|---|---|---|
| Start | 0 | When CPU turns on or resets |
| Hardware interrupt | 1 | When interupt mask is set and the hwint line is high |
| Software interrupt | 2 | When the instruction `int` is ran |
| Exception | 3 | When a privledged operation is ran in user mode |

When a hardware interupt is triggered, the interupt mask is set to 0, meaning no more hardware inerrupts will occur until it is unmasked.

Hardware and software interrupts, as well as exceptions, will push pc before jumping.

Start will reset all registers to 0.

All of these jumps will result in entering supervisor mode.

Check out the [Interrupts example](../../examples/interrupts.asm) for how to set up the interupt vector table.

# Addresses

Although technically the CPU is 32-bit, it is most practical to keep code in the first 2<sup>16</sup> addresses. 

This is because the instructions are fixed-width, and there is only space for 16 bits for an address or immediate, and so jumps to far away addresses would be a bit combersome, although still possible. You could still do jumps to addresses within 2<sup>8</sup> addresses by adding or subtracting to `pc`.

It is in theory possible to make it work with 32-bit addresses, but it would cause so much trouble to be not worth it. For example, you would have to do something similar to:

```asm
// some code ...
  sub pc, 1; // need to jump over the gen literal
far_away_label_address: far_away_label;
// some more code ...
// address 2^16
  ld r0, far_away_label_address;
  ld pc, r0;
// even more code ...
far_away_label:
// I want to jump here
```

> [!NOTE]
> A normal `ld pc, far_away_label;` would not work because the `ld` instruction only supports 16-bit addresses. Trying to do so would result in an assembler error notifying you that the address does not fit.

To load a piece of data from above address 2<sup>16</sup>, you would need to get the address in a register by whatever means. The easiest would be to just simply keep a global variable for the address in the code segement, such as:

```asm
ld r0, far_away_variable_address;
ld r0, *r0;

far_away_variable_address: 0x12345678;
```

> [!NOTE]
> A normal `ld r0, *0x12345678;` would not work because the `ld` instruction only supports 16-bit addresses. Trying to do so would result in an assembler error notifying you that the address does not fit. 
