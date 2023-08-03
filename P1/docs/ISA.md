# P-1 Instruction Set Architecture

P-1 is a simple 8-bit load/store stack-based CPU. It supports
2 memory pages of 256 bytes each, one for the code, one for 
the data. Both address and data buses are 8-bit and instructions
are 8-bit wide.

## Stack computational model

A Stack Machine is a computational model that uses a last-in, first-out stack to hold short-lived temporary values.
P-1 provides software developer with four slots deep stack, plus one more slot for the previous operation value.

|-------|
|   T   |
|-------|
|   Z   |
|-------|
|   Y   |
|-------|
|   X   |
|-------|
|  X_0  |
|-------|

Registers `X` and `Y` are primary operational registers, connected to the ALU, with `X` storing 
*first* operand and `Y` storing second operand. On operation with `X` or `X`/`Y` registers 
result of the operation is stored in the register `X` and previous values of `X` is send to
`X_0`. In case of binary operation value of `Y` is also consumed, value of `Z` is copied to
 the register `Y` and value of `T` is copied to the register `Z`. 

## Intermediate register file

To avoid unnecessary loads and stores between the memory and the stack, an additional 
register file, consisting of 15 registers `R1..R15` is provided. Register file is not connected to the 
ALU, but allows exchange between register `X` and registers `R1..R15`.

Additionally registers `R1..R15` are used in the indirect memory operations or branching operations. 
While being used in the indirect operations, some of those registers provide automatic increment or
decrement of value. `R1..R5` will automatically decrease the value by one while being used in the 
indirect operation and `R11..R15` will automatically increment the valu byt one while being used in the
indirect operation. Registers `R6..R10` do not change they value automatically.

## Hardware callstack

To support procedural call a hardware callstack is provided. It allows software developer to make a 
procedure call, automatically store the address of the callsite and return to it. The callstack is 
limited to the four levels of call nesting and it is developer's responsibility to not to make more
than four levels of calls. On a fifth call level the CPU will halt.

## Instructions and instructions formats

There are five formats of the instructions:

* 00000000 - NOP. This is the only command that uses this format. On NOP nothing happens, 
  but internal program counter is incremented.
* 00iiiiii - Load immediate. Loads a value between 1 and 63 (inclusive) to the register `X`. Other 
  stack registers are kept intact.
* 1100oooo - Register-less operations (see list below).
* 010oiiii - Partial load of immediate (see list below).
* oooorrrr - Operations with register operand (see list below).

### Register-less operations

Those operations only use stack registers and do not require any operands.

* 11000000 - PTS. Pushes the stack. `X` is copied to `Y`, `Y` is copied to `Z`,
  `Z` is copied to `T`. Value of `T` disappears and value of `X_0` is untouched.
* 11000001 - EXY. Exchanges values between registers `X` and `Y`. Value of `X_0` is untouched.
* 11000010 - RTS. Rotates the stack. `T` is copied to `Z`, `Z` is copied to `Y`, 
  `Y` is copied to `X`, `X` is copied to `T`. No value is erased, value of `X_0` is untouched.
* 11000011 - RPV. Copies value from `X_0` to `X`, `X` is copied to `Y`, `Y` is copied
  to `Z`, `Z` is copied to `T`, value of `T` is erased.
* 11000100 - ADD. Adds value of `X` to the value of `Y` and stores result to the `X` register.
* 11000101 - SUB. Subtracts value of `Y` from the value of `X` and stores result to the `X` register.
* 11000110 - MUL. Muliplies value of `X` to the value of `Y` and stores result to the `X` register. 
  This opcode is reserved for the future use.
* 11000111 - LSH. Shift value of `X` one position to the left and stores result to `X`.
* 11001000 - RSH. Shift value of `X` one position to the right and stores result to `X`.
* 11001001 - AND. Applies bitwise AND operation between registers `X` and `Y` and stores result to `X`.
* 11001010 - OR. Applies bitwise OR operation between registers `X` and `Y` and stores result to `X`.
* 11001011 - XOR. Applies bitwise XOR operation between registers `X` and `Y` and stores result to `X`.
* 11001100 - NOT. Applies bitwise NOT operation to the register `X` and stores result to `X`.
* 11001101 - RET. Jumps to the latest address stored in the hardware call stack.
* 11001110 - CLR. Write `0` to register `X`.
* 11001111 - POP. Pops the stack. Value of `X` disappears, value of `Y` is copied to `X`, value of `Z` is copied 
                  to `Y` and value of `T` is copied to `Z`.
In case stack is empty CPU will halt.
* 11001110 - CLR. Writes value 0 to the register `X`. As all zeroes are reserved for NOP, a separate command is required.

Instructions ADD, SUB, MUL, LSH, RSH, AND, OR, XOR, NOT copy operand from the `X` register to the `X_0` registers.

Instructions ADD, SUB, NUL, AND, OR, XOR will cause copying of value from `Z` to `Y` and from `T` to `Z`.

#### Partial load of immediates

For values between 1 and 63 it is recommended to use `load immediate` shortcut, that loads a value to the `X` 
in one instruction. For the other values, as we are limited in the width of command, values of `X` are load in two 
parts: upper and lower nibbles of `X` separately.

* 0100iiii - LUP. Loads four iiii bits to the leftmost four bits of the register `X`. Rightmost four bits of 
  register `X` are kept untouched.
* 0101iiii - PUP. Loads four iiii bits to the rightmost four bits of the register `X`. Leftmost four bits of 
  register `X` are kept untouched.

#### Register addressing commands

* 0110rrrr - RVR. Copies value from the register rrrr to the register `X`. In case rrrr equals to 0000, nothing happens.
* 1101rrrr - WVR. Copies value from the register `X` to the register rrrr. In case rrrr equals to 0000, nothing happens.
* 1000rrrr - LD. Loads value from the memory at address in register rrrr to the register `X`. 
* 1001rrrr - ST. Stores value from the register `X` to the memory at address in register rrrr.
* 1010rrrr - JZ. Jumps to the address stored at register rrrr in case value of `X` is zero. 
* 1011rrrr - JMP. Jumps to the address stored at register rrrr unconditionally. 
* 1111rrrr - CALL. Jumps to the address stored at register rrrr unconditionally and stores *next* program counter value in the hardware call stack. 
* 1110rrrr - JGT. Jumps to the address stored at register rrrr in case value of `X` is greater than zero.

For commands LD, ST, JZ, JMP, SUB, JGT when rrrr is equal to 0000, register `Y` is used as address source.

For all commands in that section, except RVR and WVR, when registers R1..R5 are used as rrrr, value of those register will be automatically decremented after the operation.
Same way, when registers R11..R15 are used as rrrr, value of those register will be automatically incremented after the operation.

Software engineer is required to manually track the amount of nested procedure call with SUB instruction and ensure that max four level are used.