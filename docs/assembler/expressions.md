# Compile Time Expressions

The assembler contains the ability to evaluate expression at compiile time.
These expressions do not have any cost in the runtime of the program.

For example:

``` asm
1 * 2 * (3 / 4 >>> 2) << 1;
```

This example will compile to a machine code which contains the literal value of the expression. This is an example of a Gen Literal, which is discribed in the [Gen Literals section](#gen-literals).

You may also include expression in constant identifier assignments and instruction operands:

``` asm
value = 4 * 2;
  ld r1, value >> (2 * 3);
```

As shown in the previous example, expressions can contain identifiers.

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
> As seen in the [Constants document seciton on numbers](constants.md#number), a number is always unsigned. This means there is no unary negation operator, as it would result in unexpected results. If you must get the negative of a number, then use `0 - value` instead. Additionally, the division operator is always unsigned integer division.

> [!NOTE]
> The dereference operator does not actually evaluate anything; it simply converts the type into an indirect type, as described in the [Constants document on the indirect type](constants.md#indirect). The reference operator simply gets whatever is inside and indirect type and converts it to a number, and so also does not actually evaluate anything.

Operations are evaluated left-to-right if the precedence is the same.

Additionally, you may use parentheses (`()`) to explicity set the precedence of the operations.

An expression need not contain an operation, so these are also considered expressions:

``` asm
3;
"Hello world!";
value;
```

In this example, the expression `3` evaluates to the number `3`.

## Result Types

Every expression evaluates to some type. These types are listed below:

### Number

Numbers are unsigned 32 bit integers. Chars are also considered a number, as they are the UTF-32 encoding of the character. 

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

Example: `"Hello world!\n"`, `"👻"`, and `"null terminated!\0"`

#### String Escapes

The following table enumerates the possible escape characters:

| Escape | Description
|---|---|
| `\\` | Backslash |
| `\"` | Double quote |
| `\0` | Null |
| `\t` | Horizontal tab |
| `\n` | Line feed |
| `\r` | Carriage return |

### Register

Registers are simply the registers as described in the [ISA document](../cpu/isa.md#public-registers).

Example: `r1`, `pc`, and `lr`

### Register Offset

Register offsets are simply a register +/- some offset. 

Example: `r1 + 3`, `pc - 2`, and `fp + 1`

### Indirect

indirects are simply dereferenced values. 

Example: `*r1`, `*(r1 + 3)`, and `*123`

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

``` asm
value = 123;
  value; // 123 will now be included in the machine code at this location
string = "hello world!";
  string + value; // the string of UTF-8 values of "hello world!123" will appear in the machine code
  "another string"; // this string will also appear in the machine code immediately after the last string
```
