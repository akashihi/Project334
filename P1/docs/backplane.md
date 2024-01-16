# P-1 Backplane connectors allocation

The P-1 CPU is implemented as a set of modules, connected by a single (almost) passive backplane.
Backplane provides passive data buses connection and power distribution. 
Modules connect to a backplane using 96 pins (3x32) 41612 C connectors. There is no rack, neither
chassis system provided, instead modules are connected to backplane on one side and screwed together
using M3 bolts and stand on the other side. Modules follow a Half-Eurocard standard (80mmx100mm) 
with connectors being attached to the long side.

## P-1 Power distribution

P-1 is powered by 24V DC power supply. Backplane provides conversion from 24V to 5V and distributes
both power rails to each module. Some modules that require 3V voltage are equipped with their own
5V-3V DCDC.

## Signals present on a backplane connector

* CLK - Driven by MCU module
* RESET - Driven by MCU module
* MEM_DATA[8] - Tri state signal driven by both memory interface module and MCU module
* MEM_ADDRESS[8] - Driven by memory interface module
* MEM_RAM - Driven by memory interface module
* MEM_RW - Driven by memory interface module
* X_OUT[8] - Driven by the stack module
* Y_OUT[8] - Driven by the stack module
* X_IN[8] - Open drain bus with backplane providing pullup resistors
* STACK_OP[4] - Driven by the decoder
* ALU_OP[4] - Driven by the decoder
* REGISTER_ID[4] - Driven by the decoder
* INDIRECT_OP - Driven by the decoder
* RVR - Driven by the decoder
* WVR - Driven by the decoder
* ADDRESS_OUT[8] - Open drain bus with backplane providing pullup resistors
* ERROR - Driven by the substack module
* CALL - Driven by the decoder
* RET - Driven by the decoder
* OP_ADDRESS[8] - Driven by the fetch module
* OPCODE[8] - Driven by the fetch module
* MEM_OP_DATA - Driven by the decoder
* INSTRUCTION[8] - Driven by the memory interface module
* SET_PC - Driven by the decoder
* MEM_OP_RW -  - Driven by the decoder

In total 96 signals 😱

## Section A

This section is dedicated to the ALU signals, such as X_IN,X_OUT etc

* A1..A8 - X_OUT bus
* A9 - GND
* A10..A17 - Y_OUT bus
* A18 - GND
* A19..A26 - X_IN bus
* A27 - GND
* A28..A31 - ALU_OP
* A32 - +5V


## Section B

This section handles control signals and external signals.

* B1 - RST
* B2 - CLK
* B3 - GND
* B4 - MEM_RAM
* B5 - MEM_RW
* B6 - GND
* B7..B14 - MEM_DATA bus
* B15 - GND
* B16..B23 - MEM_ADDRESS bus
* B24 - GND
* B25..B32 - INSTRUCTION bus


## Section C

Last sections transmits internal control signals and instruction related data

* C1..C4 - STACK_OP
* C5 - GND
* C6..C9 - REGISTER_ID
* C10 - GND
* C11 - INDIRECT_OP
* C12 - RVR
* C13 - WVR
* C14 - ERROR
* C15 - CALL
* C16 - RET
* C17 - GND
* C18..C25 - ÀDDRESS_OUT bus
* C26 - GND
* C27 - +24V
* C28 - GND
* C29 - +5V
* C30 - GND
* C31 - +24V
* C32 - +5V

## Missing signals

Signals `OP_ADDRESS[8]`, `OPCODE[8]`, `INSTRUCTION[8]`, `MEM_OP_DATA`, `SET_PC`, `MEM_OP_RW` are not exposed to the backplane. Three modules that generate or use those signals, `fetch`, `memory interface` and `decode` work close to each other and will be located on a same PCB, 
thus removing the need to expose those signals
