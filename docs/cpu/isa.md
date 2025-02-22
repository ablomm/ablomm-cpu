# Public Registers
There are 12, 32-bit, general purpose register (r0 to r11)
> [!NOTE]
> r11 has no special meaning to the CPU, although, the assembler will use "fp" (frame pointer) as an alias to r11.

There are 4 special purpose registers with varying widths. The table below enumerates all registers and their purpose.

| Register | Code | Description | Width |
|----------|------|-------------|-------|
| r0 | 0 | General purpose | 32 |
| r1 | 1 | General purpose | 32 |
| r2 | 2 | General purpose | 32 |
| r3 | 3 | General purpose | 32 |
| r4 | 4 | General purpose | 32 |
| r5 | 5 | General purpose | 32 |
| r6 | 6 | General purpose | 32 |
| r7 | 7 | General purpose | 32 |
| r8 | 8 | General purpose | 32 |
| r9 | 9 | General purpose | 32 |
| r10 | 10 | General purpose | 32 |
| r11 | 11 | General purpose | 32 |
| status | 12 | state of the CPU; conditions, interupt mask, and mode | 6 |
| sp | 13 | stack pointer; points to last item in stack, and grows down | 32 |
| lr | 14 | link register; is set to previous pc value if pc is explicitly written to | 32 |
| pc | 15 | program counter; points to next instruction to run | 32 |

## Status Register
Among all the special purpose registers, the status register is the only register with a different width.

The layout of the status register is as follows:
| 0 | 1 | 2 | 3 | 4 | 5 |
|---|---|---|---|---|---|
| N | Z | C | V | I | M |

The status register is made of various flags. These flags are sumarized in the table below:

| Flag | Description |
|------|-------------|
| N | The last ALU operation with S=1 resulted in a negative number |
| Z | The last ALU operation with S=1 resulted in zero |
| C | The last ALU operation that with S=1 resulted in a carry |
| V | The last ALU operation that with S=1 resuted in a signed overflow |
| I | Hardware interrupt mask, i.e., I=0 means no hardware interrupts will occur |
| M | If set to 0, CPU is in supervisor mode, if set to 1, CPU is in user mode | 

The first four of these flags are used for conditional execution. The following table details the condition, the corresponding NZCV flags, and the condition code.

### Conditions
| Condition | Code | Description | NZCV Expression |
|-----------|------|-------------|-----------------|
| none | 0 | always executes | true |
| eq | 1 | `sub.t x, y` where x == y | Z |
| ne | 2 | `sub.t x, y` where x != y | !Z |
| ult | 3 | `sub.t x, y` where x and y are unsigned, and x < y  | !C |
| ugt | 4 | `sub.t x, y` where x and y are unsigned, and x > y  | C && !Z |
| ule | 5 | `sub.t x, y` where x and y are unsigned, and x <= y  | !C || Z |
| uge | 6 | `sub.t x, y` where x and y are unsigned, and x >= y  | C |
| slt | 7 | `sub.t x, y` where x and y are signed, and x < y  | N !== V |
| sgt | 8 | `sub.t x, y` where x and y are signed, and x > y  | !Z && (N == V) |
| sle | 9 | `sub.t x, y` where x and y are signed, and x <= y  | Z || (N != V) |
| sge | 10 | `sub.t x, y` where x and y are signed, and x >= y  | N == V |

# Instruction
There is only 10 instruction from the CPU's perspective. The ALU instruction is generic and works with all supported ALU operations. The instructions are enumerated in the following table:
| Instruction | Code | Description |
|-------------|------|-------------|
| NOP | 0 | No operation |
| LD | 1 | register = *address |
| LDR | 2 | register = *register |
| LDI | 3 | register = immediate |
| ST | 4 | *address = register |
| STR | 5 | *register = register |
| PUSH | 6 | *(--sp) = register |
| POP | 7 | register = *(sp++) |
| INT | 8 | Software interrupt;  jumps to address shown in the [Interupt Vector Table](#interrupt-vector-table) section
| ALU | 0xf_ | Performs an ALU operation as shown in the [ALU Operations](#alu-operations) section

> [!NOTE]
> The assembler has a different set of instructions that will mapped to these instructions. E.g. `ld r1, 123` gets mapped to a LDI instuction.

## Layout
Each instruction is 32 bits; there is no variable length instructions.

The ALU operations can be found in the [ALU Operations](#alu-operations) section.

The condition codes can be found in the [Conditions](#conditions) section.

The registers codes can be found in the [Public Reigsters](#public-registers) section.

 <table>
  <tr>
    <th>Op</th>
    <th>0</th>
    <th>1</th>
    <th>2</th>
    <th>3</th>
    <th>4</th>
    <th>5</th>
    <th>6</th>
    <th>7</th>
    <th>8</th>
    <th>9</th>
    <th>10</th>
    <th>11</th>
    <th>12</th>
    <th>13</th>
    <th>14</th>
    <th>15</th>
    <th>16</th>
    <th>17</th>
    <th>18</th>
    <th>19</th>
    <th>20</th>
    <th>21</th>
    <th>22</th>
    <th>23</th>
    <th>24</th>
    <th>25</th>
    <th>26</th>
    <th>27</th>
    <th>28</th>
    <th>29</th>
    <th>30</th>
    <th>31</th>
  </tr>
  <tr>
    <td>NOP</td>
    <td colspan="4" rowspan="11">Condition Code</td>
    <td colspan="8">Instruction Code = 0</td>
    <td colspan="24" rowspan="2">Unused</td>
  </tr>
  <tr>
    <td>INT</td>
    <td colspan="8">Instruction Code = 8</td>
  </tr>
  <tr>
    <td>LD</td>
    <td colspan="8">Instruction Code = 1</td>
    <td colspan="4" rowspan="2">Register A</td>
    <td colspan="16" rowspan="2">Address</td>
  </tr>
  <tr>
    <td>ST</td>
    <td colspan="8">Instruction Code = 4</td>
  </tr>
  <tr>
    <td>LDR</td>
    <td colspan="8">Instruction Code = 2</td>
    <td colspan="4" rowspan="2">Register A</td>
    <td colspan="4" rowspan="2">Register B</td>
    <td colspan="12" rowspan="2">Offset</td>
  </tr>
  <tr>
    <td>STR</td>
    <td colspan="8">Instruction Code = 5</td>
  </tr>
  <tr>
    <td>LDI</td>
    <td colspan="8">Instruction Code = 3</td>
    <td colspan="4">Register A</td>
    <td colspan="16">Immediate</td>
  </tr>
  <tr>
    <td>PUSH</td>
    <td colspan="8">Instruction Code = 6</td>
    <td colspan="4" rowspan="2">Register A</td>
    <td colspan="16" rowspan="2">Unused</td>
  </tr>
  <tr>
    <td>POP</td>
    <td colspan="8">Instruction Code = 7</td>
  </tr>
  <tr>
    <!-- ALU -->
    <td rowspan="2">ALU</td>
    <td colspan="4" rowspan="2">Instruction Code High = 15</td>
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
> "Register A", "Register B", and "Register C" correspond to A, B, and C in the table below, although, if I=1 (immediate bit it set), then C = immediate, and if R=1 (reverse bit is set), then "Register B" corresponds to C, and "Register C" corresponds to B (i.e., reversed), and if Ln=1 (Loadn bit is set), then A is not set to any register.

| Op | Code | Pseudo Code |
|----|------|-------------|
| PASS | 0 | A = C |
| AND | 1 | A = B & C |
| OR | 2 | A = B \| C |
| XOR | 3 | A = B ^ C |
| NOT | 4 | A = ~C |
| ADD | 5 | A = B + C |
| ADDC | 6 | A = B + C + carry |
| SUB | 7 | A = B - C |
| SUBB | 8 | A = B - C - borrow |
| NEG | 9 | A = -C |
| SHL | 10 | A = B << C |
| SHR | 11 | A = B >> C |
| ASHR | 12 | A = B >>> C |

> [!NOTE]
> Borrow is simply ~carry

## ALU Flags
You may have noticed the ALU CPU instruction contains four bits named "I", "R", "Ln", and "S". These flags modify the ALU's behaviour. The following table sumarizes these behaviours:

| Flag | Description | Purpose |
|------|-------------|---------|
| I | Immediate flag; if set to 1, then the last 16 bits of the instruction is interpreted as an immediate value | So we can do stuff like `add, R1, 123` |
| R | Reverse flag; if set to 1, then the role of Register B, and Register C (or immediate) is flipped | So we can do stuff like `sub 123, R1`; not really useful for if I=0 |
| Ln | Loadn flag; if set to 1, then do not load the result of the operation into Register A | So we can do stuff like `sub.t, r1, r2`; not really useful for if S=0, but note the .t is like `TST` is other ISAs |
| S | Set status flag; if set to 1, then sets the status register with it's various flags | So we can do stuff like `sub.s, r1, r2` |

## Interrupt Vector Table
The addresses of various locations in memory the CPU may jump to in each case are as follows:

| Purpose | Address | When |
|---------|---------|------|
| Start | 0 | When CPU turns on or resets |
| Hardware interrupt | 1 | When interupt mask is set and the hwint line is high |
| Software interrupt | 2 | When the instruction `int` is ran |
| Exception | 3 | When a privledged operation is ran in user mode |

When a hardware interupt is triggered, the interupt mask is set to 0, meaning no more hardware inerrupts will occur until it is unmasked.

Hardware and software interrupts, as well as exceptions, will push pc and lr before jumping.

Start will reset all registers to 0.

All of these jumps will result in entering supervisor mode.
