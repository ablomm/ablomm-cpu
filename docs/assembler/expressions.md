# Assembly-time Expressions

The assembler contains the ability to evaluate expressions at assembly time.

These expressions do not have any cost on the run time of the program.

For example:

```c
1 * 2 * (16 / 4 >>> 2) << 1;
```

This example will assemble to machine code which contains the literal value of the expression. This is an example of a Gen Literal, which is described in the [Gen Literals section](#gen-literals).

You may also include expressions in [assignments](assignments.md) and [instruction operands](instructions.md#operands):

```c
value = 4 * 2;
ld r1, value >> (2 * 3);
```

As shown in the previous example, expressions can contain identifiers.

Identifiers are explained further in the [Assignments document](assignments.md) and [Labels document](labels.md).

Expressions can also contain registers:

```c
ld r1, *(r2 + 3 * 2);
value = r4 - 3;
ld r2, *value;
```

> [!NOTE]
> If a register is in an expression, it is only valid if the expression evaluates to a register +- some number. This is called a register offset, as outlined in the [Result Types section](#register-offset).

 
> [!NOTE]
> A register in an expression does not evaluate the value of the register. A register in an expression acts more like a define in C. All expressions are evaluated at assembly time, and so cannot evaluate any run-time values such as registers. This is explained more in the [Assignments document](assignments.md).

A list of all the possible operators in an expression are detailed in the table below in the order of precedence:

<table>
<tr>
<th>Precedence</th>
<th>Operator</th>
<th>Description</th>
<th>Example</th>
</tr>
  
<tr>
<td rowspan="3">1</td>
<td>
      
`&`
      
</td>
<td>Reference</td>
<td>

`&*123`

</td>
</tr>
  
<tr>
<td>

`*`
      
</td>
<td>Dereferance</td>
<td>

`*123`
      
</td>
</tr>
  
<tr>
<td>

`~`

</td>
<td>Bitwise NOT</td>
<td>

`~123` 

</td>
</tr>

<tr>
<td rowspan="3">2</td>
<td>

`*`
      
</td>
<td>Multiplication</td>
<td>

`123 * 321`
      
</td>
</tr>

<tr>
<td>

`/`
      
</td>
<td>Division</td>
<td>

`123 / 2`
      
</td>
</tr>


<tr>
<td>

`%`
      
</td>
<td>Modulo</td>
<td>

`123 % 2`
      
</td>
</tr>

<tr>
<td rowspan="2">3</td>
<td>

`+`
      
</td>
<td>Addition</td>
<td>

`123 + 321`
      
</td>
</tr>

<tr>
<td>

`-`
      
</td>
<td>Subtraction</td>
<td>

`123 - 23`
      
</td>
</tr>

<tr>
<td rowspan="3">4</td>
<td>

`<<`
      
</td>
<td>Left shift</td>
<td>

`123 << 2`
      
</td>
</tr>

<tr>
<td>

`>>>`
      
</td>
<td>Arithmetic right shift</td>
<td>

`123 >>> 2`
      
</td>
</tr>

<tr>
<td>

`>>`
      
</td>
<td>Logical right shift</td>
<td>

`123 >> 2`
      
</td>
</tr>

<tr>
<td>5</td>
<td>

`&`
      
</td>
<td>Bitwise AND</td>
<td>

`123 & 321`
      
</td>
</tr>

<tr>
<td>6</td>
<td>

`^`
      
</td>
<td>Bitwise exclusive OR</td>
<td>

`123 ^ 321`
      
</td>
</tr>

<tr>
<td>7</td>
<td>

`|`
      
</td>
<td>Bitwise OR</td>
<td>

`123 | 321`
      
</td>
</tr>

</table>

> [!NOTE]
> As seen in the [Number section](#number), a number is always unsigned. This means there is no unary negation operator, as it would result in unexpected results. If you must get the 2's compliment of a number, then use `0 - value` instead. Additionally, the division operator is always unsigned integer division.

> [!NOTE]
> The dereference operator does not actually evaluate anything; it simply converts the type into an indirect type, as described in the [Indirect section](#indirect). The reference operator simply gets whatever is inside and indirect type and converts it to a number, and so does not actually evaluate anything.

Operations are evaluated left-to-right if the precedence is the same.

Additionally, you may use parentheses (`()`) to explicitly set the precedence of the operations.

An expression need not contain an operation, so `3`, `"Hello world!"`, and `value` are also considered expressions:

In this example, the expression `3` evaluates to the number `3`.

The expression `"Hello world!"` evaluates to the string `"Hello world!"`, etc.

## Result Types

Every expression evaluates to some type. These types are listed below:

### Number

Numbers are unsigned 32-bit integers. Chars are also considered a number, as they are the UTF-32 encoding of the character. 

Example: `123`, `0x123`, and `'a'`.

#### Char Escapes

The following tables enumerates the possible escape characters:

| Escape | Value | Description
|---|---|---|
| `\0` | `0x0` | Null |
| `\t` | `0x9` | Horizontal tab |
| `\n` | `0xa` | Line feed |
| `\r` | `0xd` | Carriage return |

> [!NOTE]
> There is no escapes for `'` and `\` because these can be expressed as `'''` and `'\'` respectively.

### String

Strings are a sequence of UTF-8 encodings of characters and are not null terminated (null termination must be done explicitly). Strings allow for escaping special control characters with `\`. 

Example: `"Hello world!\n"`, `"👻"`, and `"null terminated!\0"`.

#### String Escapes

The following table enumerates the possible string escape characters:

| Escape | Description
|---|---|
| `\\` | Backslash |
| `\"` | Double quote |
| `\0` | Null |
| `\t` | Horizontal tab |
| `\n` | Line feed |
| `\r` | Carriage return |

### Register

Registers hold data in the CPU to use for instructions.

These registers are mapped to CPU registers as seen in the [ISA document](../cpu/isa.md#public-registers).

All registers are 32-bits, except the status register, which is 6-bits. Refer to the [ISA document](../cpu/isa.md#status-register) for the layout of the status register.

The list of registers allowed in the assembly is as follows:

<table>
  
<tr>
<th>Register</th>    
<th>Description</th>
<th>Example</th>
<th>CPU register</th>
</tr>

<tr>
<td>r0</td>
<td>General purpose</td>
<td>

`ld r0, 123;`

</td>
<td>R0</td>
</tr>

<tr>
<td>r1</td>
<td>General purpose</td>
<td>

`ld r1, r0;`

</td>
<td>R1</td>
</tr>

<tr>
<td>r2</td>
<td>General purpose</td>
<td>

`add r2, 123;`

</td>
<td>R2</td>
</tr>

<tr>
<td>r3</td>
<td>General purpose</td>
<td>

`sub r2, 123;`

</td>
<td>R3</td>
</tr>

<tr>
<td>r4</td>
<td>General purpose</td>
<td>

`sub.t 123, r4;`

</td>
<td>R4</td>
</tr>

<tr>
<td>r5</td>
<td>General purpose</td>
<td>

`push r5;`

</td>
<td>R5</td>
</tr>

<tr>
<td>r6</td>
<td>General purpose</td>
<td>

`shr r6, 2;`

</td>
<td>R6</td>
</tr>

<tr>
<td>r7</td>
<td>General purpose</td>
<td>

`shr.s r7, r2;`

</td>
<td>R7</td>
</tr>

<tr>
<td>r8</td>
<td>General purpose</td>
<td>

`ld r8, *123;`

</td>
<td>R8</td>
</tr>

<tr>
<td>r9</td>
<td>General purpose</td>
<td>

`ld *123, r9;`

</td>
<td>R9</td>
</tr>

<tr>
<td>r10</td>
<td>
  
General purpose (if not using as `fp`)

</td>
<td>

`ld r10, r5;`

</td>
<td rowspan="2">R10</td>
</tr>

<tr>
<td>fp</td>
<td>
  
Frame pointer; aliases to `r10`

</td>
<td>

`ld r0, *(fp + 3);`

</td>
</tr>

<tr>
<td>status</td>
<td>
  
State of the CPU; conditions, interrupt mask, and mode. Refer to the [ISA document](../cpu/isa.md#status-register) for more information

</td>
<td>

`or status, 0b10;`

</td>
<td>STATUS</td>
</tr>

<tr>
<td>sp</td>
<td>Stack pointer; points to last item in stack, and grows down</td>
<td>

`sub sp, 10;`

</td>
<td>SP</td>
</tr>

<tr>
<td>lr</td>
<td>
  
Link register; is set to previous `pc` value if `pc.link` is written to

</td>
<td>

`ld pc, lr;`

</td>
<td>LR</td>

<tr>
<td>pc.link</td>
<td>
  
Program counter with link; A pseudo register used to load `pc` and load `lr` with the previous `pc` value (much like some ISAs' jump with link). Use this register to jump and set `lr`

</td>
<td>

`ld pc.link, print;`

</td>
<td>PCLINK</td>
</tr>

<tr>
<td>pc</td>
<td>
  
Program counter; points to next instruction to run. Use this register to jump without setting `lr`

</td>
<td>

`ld pc, end;`

</td>
<td>PC</td>
</tr>

</tr>

</table>

Example: `r1`, `pc`, and `lr`.

### Register Offset

Register offsets are simply a register +/- some offset. 

Example: `r1 + 3`, `pc - 2`, and `fp + 1`.

### Indirect

Indirects are simply dereferenced values. 

Example: `*r1`, `*(r1 + 3)`, and `*123`.

## Type Specific Implementations of Operators

Some operators only work on some types. For example, you cannot use division on the string type.

Additionally, some operators are implemented differently for different types. The only case of this is the addition and subtraction operators.

Addition:
- For numbers, it is just the addition of the numbers
- For a register and a number, it is a new register offset type
- For a register offset and a number, it is a register offset with the offset added with the number
- For strings, it is string concatenation

Subtraction:
- For numbers, it is just the subtraction of the numbers
- For a register and a number, it is a new register offset type
- For a register offset and a number, it is a register offset with the offset subtracted from the number

## Gen Literals

It is possible to include the value of an expression within the generated machine code. This functionality only exists for numbers and strings.

To generate an expression into the machine code, simply write out the expression directly.

For example:

```c
value = 123;
value; // 123 will now be included in the machine code at this location
string = "hello world!";
string + value; // the string of UTF-8 values of "hello world!123" will appear in the machine code
"another string"; // this string will also appear in the machine code immediately after the last string
```
