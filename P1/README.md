# P1 - Processor One 

Simple 8-bit stack based CPU built with standard logic ICs.

## Contents

- [Project organization](#project-organization)
- [Architecture](#architecture)
- [Toolchain](#toolchain)
- [Logical design]
- [Schematics and PCB]
- [Authors]
- [License]

## Project Organization

* `benchmark` - Host system benchmark code. Implements several matrix operation on 8-bit
unsigned integers.
* `docs` - More detailed documentation on the CPU design and Instruction Set Architecture.
* `prgs` - Programs written for P1 CPU using P1 mnemonics
* `tb` - Test vector generators for ALU and branching unit. 
* `tests` - Test programs in P1 mnemonics that execute every single possible opcode in a predictable manner
* `tooling` - Mnemonics translation tool `p1asm` and CPU emulator `p1emu`

CPU logical design is done in [Logisim Evolution](https://github.com/logisim-evolution/logisim-evolution), the main circuit is 
stored in `P1.circ` file. 

CPU physical design is done in [KiCad](https://www.kicad.org/)

Tooling is written in [Rust](https://rustup.rs/)

## Architecture

P1 is a simple 8-bit, stack based CPU with 8-bit address and data buses. The main characteristic of it as it is a 
stack-based CPU, meaning that all operations are executed on a hardware stack, not on addressable registers.

CPU provides four items stack with registers `X`,`Y`,`Z` and `T`, plus a "previous X value" register `X0`. 
All operations are applied to the values of registers `X` and `Y` and operations results are stored back to register `X`. 
CPU provides wide set of stack specific operations.

For an intermediate data storage CPU provides 15 addressable registers, which can be used as a temporary data storage. 
Those registers can not be used in any other operation, except reading or updating their value. However, those 
registers could be used with branching instructions, procedure calls or data accesses as the source of address, 
thus allowing indirect operations. Some of those registers will automatically increase of decrease their values upon 
indirect call, thus allowing automatic sequential reads/write of memory.

Speaking of memory, CPU support 256 bytes of RAM **and** 256 bytes of program ROM. Program memory and data memory are in 
different address spaces, however they share the same address and data buses.

More details on the CPU instructions and behaviour details can be obtained from the [Instruction Set Architecture documentation](/docs/ISA.md).

## Toolchain

What toolchain? There is no real toolchain, but rather a couple of tools that help you write and debug programs for P1 cpu.

### p1asm

Located under `tooling\p1asm` is a Rust tool to convert a program written in P1 autocode to the executable binary. `p1asm` tool
 accepts a text source file and generates a binary file, replacing source file's extension with `p1b`. Example:

````shell
>cargo run hello_world.p1s
Converting hello_world.p1s input file to "hello_world.p1b" output binary
LUP 248 -> 01001111
PUP 248 -> 01011000
WVR R15 -> 11011111
LUP 72 -> 01000100
PUP 72 -> 01011000
ST R15 -> 10011111
LUP 105 -> 01000110
PUP 105 -> 01011001
ST R15 -> 10011111
SET 32 -> 00100000
ST R15 -> 10011111
LUP 116 -> 01000111
PUP 116 -> 01010100
ST R15 -> 10011111
LUP 104 -> 01000110
PUP 104 -> 01011000
ST R15 -> 10011111
LUP 101 -> 01000110
PUP 101 -> 01010101
ST R15 -> 10011111
LUP 114 -> 01000111
PUP 114 -> 01010010
ST R15 -> 10011111
LUP 101 -> 01000110
PUP 101 -> 01010101
ST R15 -> 10011111                                                                                                                                                                                                                                                                                                          
````

`p1asm` expects an input file with a single opcode per line. For the register addressing opcodes, like `ST` please use an `R` prefix: `ST R15`.
For the register addressing opcodes, that operate on the `Y` register, please provide the register name directly: `LD Y`.

Assembly tool automatically checks that your program contains not more than 256 program steps and will fail on program memory overflow. 
Please pay attention, that you should use `SET` opcode instead of the `LUP/PUP` and `load immediate`. Assembly tool will automatically
convert those set call to either pair of `LUP/PUP` or to the short load of the immediate.

Unfortunately otherwise the tool is pretty limited and doesn't provide neither linking nor other automation. It is recommended to use supplied
`prgs\template.p1s` for tracking the addresses and track the content of the stack manually, like it is done in `tests\test_alu_ops.p1s`.

### p1emu

Second tool is located under `tooling\p1emu` and emulates the instruction set architecture of the P1 CPU. For the more precise cycle-perfect simulation
one can use the [logical design simulation](#logical-design) with logisim.

However, neither hardware implementation nor logisim provide convenient debug interface and ease of use. On the other hand the `p1emu` tool 
provides you with a simple way to run a P1 software on the host system and dumps the CPU state on each opcode processed. Additionally, 
it allows you to load pre-saved RAM or store it to the file at the end of the execution. The `p1emu` tool treats the NOP opcode as the execution
completion marker, unlike logisim model (and, obviously, hardware), which just skips to the next opcode. Example:

````shell
>cargo run hello_world.p1b -w hello_world.ram
Loading program code from hello_world.p1b
Loaded 26 bytes
PC: 000, X0: 000, X: 000, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 000 
Output: 000 000 000 000 000 000 000 000 000 000  : 
Opcode: 01001111, Instruction: LUP 240
PC: 001, X0: 000, X: 240, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 000 
Output: 000 000 000 000 000 000 000 000 000 000  : 
Opcode: 01011000, Instruction: PUP 8
PC: 002, X0: 000, X: 248, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 000 
Output: 000 000 000 000 000 000 000 000 000 000  : 
Opcode: 11011111, Instruction: WVR 15
PC: 003, X0: 000, X: 248, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 248 
Output: 000 000 000 000 000 000 000 000 000 000  : 
Opcode: 01000100, Instruction: LUP 64
PC: 004, X0: 000, X: 072, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 248 
Output: 000 000 000 000 000 000 000 000 000 000  : 
Opcode: 01011000, Instruction: PUP 8
PC: 005, X0: 000, X: 072, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 248 
Output: 000 000 000 000 000 000 000 000 000 000  : 
Opcode: 10011111, Instruction: ST 15
PC: 006, X0: 000, X: 072, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 249 
Output: 000 000 072 000 000 000 000 000 000 000  : 
Opcode: 01000110, Instruction: LUP 96
PC: 007, X0: 000, X: 104, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 249 
Output: 000 000 072 000 000 000 000 000 000 000  : 
Opcode: 01011001, Instruction: PUP 9
PC: 008, X0: 000, X: 105, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 249 
Output: 000 000 072 000 000 000 000 000 000 000  : 
Opcode: 10011111, Instruction: ST 15
PC: 009, X0: 000, X: 105, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 250 
Output: 000 000 072 105 000 000 000 000 000 000  : 
Opcode: 00100000, Instruction: Immediate: 32
PC: 010, X0: 000, X: 032, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 250 
Output: 000 000 072 105 000 000 000 000 000 000  : 
Opcode: 10011111, Instruction: ST 15
PC: 011, X0: 000, X: 032, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 251 
Output: 000 000 072 105 032 000 000 000 000 000  : 
Opcode: 01000111, Instruction: LUP 112
PC: 012, X0: 000, X: 112, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 251 
Output: 000 000 072 105 032 000 000 000 000 000  : 
Opcode: 01010100, Instruction: PUP 4
PC: 013, X0: 000, X: 116, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 251 
Output: 000 000 072 105 032 000 000 000 000 000  : 
Opcode: 10011111, Instruction: ST 15
PC: 014, X0: 000, X: 116, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 252 
Output: 000 000 072 105 032 116 000 000 000 000  : 
Opcode: 01000110, Instruction: LUP 96
PC: 015, X0: 000, X: 100, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 252 
Output: 000 000 072 105 032 116 000 000 000 000  : 
Opcode: 01011000, Instruction: PUP 8
PC: 016, X0: 000, X: 104, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 252 
Output: 000 000 072 105 032 116 000 000 000 000  : 
Opcode: 10011111, Instruction: ST 15
PC: 017, X0: 000, X: 104, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 253 
Output: 000 000 072 105 032 116 104 000 000 000  :
Opcode: 01000110, Instruction: LUP 96
PC: 018, X0: 000, X: 104, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 253 
Output: 000 000 072 105 032 116 104 000 000 000  : 
Opcode: 01010101, Instruction: PUP 5
PC: 019, X0: 000, X: 101, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 253 
Output: 000 000 072 105 032 116 104 000 000 000  : 
Opcode: 10011111, Instruction: ST 15
PC: 020, X0: 000, X: 101, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 254 
Output: 000 000 072 105 032 116 104 101 000 000  : 
Opcode: 01000111, Instruction: LUP 112
PC: 021, X0: 000, X: 117, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 254 
Output: 000 000 072 105 032 116 104 101 000 000  : 
Opcode: 01010010, Instruction: PUP 2
PC: 022, X0: 000, X: 114, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 254 
Output: 000 000 072 105 032 116 104 101 000 000  : 
Opcode: 10011111, Instruction: ST 15
PC: 023, X0: 000, X: 114, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 255 
Output: 000 000 072 105 032 116 104 101 114 000  : 
Opcode: 01000110, Instruction: LUP 96
PC: 024, X0: 000, X: 098, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 255 
Output: 000 000 072 105 032 116 104 101 114 000  : 
Opcode: 01010101, Instruction: PUP 5
PC: 025, X0: 000, X: 101, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 255 
Output: 000 000 072 105 032 116 104 101 114 000  : 
Opcode: 10011111, Instruction: ST 15
PC: 026, X0: 000, X: 101, Y: 000, Z: 000, T: 000, S0: 000, S1: 000, S2: 000, S3: 000
R1: 000 R2: 000 R3: 000 R4: 000 R5: 000 R6: 000 R7: 000 R8: 000 R9: 000 R10: 000 R11: 000 R12: 000 R13: 000 R14: 000 R15: 000 
Output: 000 000 072 105 032 116 104 101 114 101  : 
Opcode: 00000000, Instruction: NOP
Stopping CPU due to: NOP treated as stop condition
Dumping memory to hello_world.ram 
````

The RAM content is pretty expected:
````shell
0000000000: 00 00 00 00 00 00 00 00 │ 00 00 00 00 00 00 00 00                                                                                                                                                                                                                                                               
0000000010: 00 00 00 00 00 00 00 00 │ 00 00 00 00 00 00 00 00                                                                                                                                                                                                                                                               
0000000020: 00 00 00 00 00 00 00 00 │ 00 00 00 00 00 00 00 00                                                                                                                                                                                                                                                               
0000000030: 00 00 00 00 00 00 00 00 │ 00 00 00 00 00 00 00 00                                                                                                                                                                                                                                                               
0000000040: 00 00 00 00 00 00 00 00 │ 00 00 00 00 00 00 00 00                                                                                                                                                                                                                                                               
0000000050: 00 00 00 00 00 00 00 00 │ 00 00 00 00 00 00 00 00                                                                                                                                                                                                                                                               
0000000060: 00 00 00 00 00 00 00 00 │ 00 00 00 00 00 00 00 00                                                                                                                                                                                                                                                               
0000000070: 00 00 00 00 00 00 00 00 │ 00 00 00 00 00 00 00 00                                                                                                                                                                                                                                                               
0000000080: 00 00 00 00 00 00 00 00 │ 00 00 00 00 00 00 00 00                                                                                                                                                                                                                                                               
0000000090: 00 00 00 00 00 00 00 00 │ 00 00 00 00 00 00 00 00                                                                                                                                                                                                                                                               
00000000A0: 00 00 00 00 00 00 00 00 │ 00 00 00 00 00 00 00 00                                                                                                                                                                                                                                                               
00000000B0: 00 00 00 00 00 00 00 00 │ 00 00 00 00 00 00 00 00                                                                                                                                                                                                                                                               
00000000C0: 00 00 00 00 00 00 00 00 │ 00 00 00 00 00 00 00 00                                                                                                                                                                                                                                                               
00000000D0: 00 00 00 00 00 00 00 00 │ 00 00 00 00 00 00 00 00                                                                                                                                                                                                                                                               
00000000E0: 00 00 00 00 00 00 00 00 │ 00 00 00 00 00 00 00 00                                                                                                                                                                                                                                                               
00000000F0: 00 00 00 00 00 00 00 00 │ 48 69 20 74 68 65 72 65          Hi there
````

The last eight bytes of the RAM are considered an _output_area_ in both emulator and hardware implementation. In the `p1emu` contents of the output
area is printed at each iteration and in hardware implementation it is sent to the screen.

### Provided software

There are two programs under the `prgs` folder:

* hello_world.p1s - puts a phrase `Hi there` to the _output_area_
* benchmark.p1s - benchmarking tool, that does couple of the matrix operations. See [benchmarking section](#benchmarking) below.

Additionally, there are a couple of test programs under `tests` folder, that call each opcode of the P1 CPU. They have been mostly used during 
logical design phase.

### Benchmarking

Or course it is fascinating to know, how slow P1 CPU is, compared to the modern CPUs. As the P1 CPU is pretty limited in everything, i came up
with a simple benchmark, based on a lot of arithmetic operations and branching: matrix operations. Benchmark take two matrices and a number,
multiplies first number by that number, then multiplies those two matrices and, finally, calculates determinant of the resulting matrix. 

The first implementation of the benchmark is a host implementaion, under `benchmark` folder. It is written in a very non-optimized way, but still does the trick.
On a modern system the execution time of the host benchmark is between 50-100 nanoseconds. 

Respective implementation of the same algorithm in P1 autocode is available under `prgs\benchmark.p1s`. As the P1 CPU doesn't have a multiplication operation,
it is emulated as a series of ADD calls at the moment. The full benchmark execution requires about 60k of CPU cycles, so the dependency between
runtime and CPU clock is linear and predictable.