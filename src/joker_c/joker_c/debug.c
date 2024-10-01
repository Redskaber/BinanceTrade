/*
* debug.c
* Bytecode Virtual Machine - Debugging functions
* 
* This file contains the implementation of the debugging functions used by the
* bytecode virtual machine.
*/

#include <stdio.h>
#include "debug.h"
#include "value.h"
#include "chunk.h"

// Forward declarations of helper functions
static uint32_t simple_instruction(const char* name, uint32_t offset);
static uint32_t constant_instruction(const char* name, Chunk* chunk, uint32_t offset);
static uint32_t constant_long_instruction(const char* name, Chunk* chunk, uint32_t offset);



void disassemble_chunk(Chunk* chunk, const char* name) {
    printf("== %s ==\n", name);
    for (uint32_t offset = 0; offset < chunk->count;) {
        // return next instruction offset
        offset = disassemble_instruction(chunk, offset);
    }
}

uint32_t disassemble_instruction(Chunk* chunk, uint32_t offset) {
    printf("%04d ", offset);

    // print line number (if changed)
    line_t current_line = get_rle_line(&chunk->lines, offset);
    if (offset > 0 && current_line == get_rle_line(&chunk->lines, offset - 1)) {
        printf("   | ");
    } else {
        printf("%4d ", current_line);
    }
    // print instruction
    uint8_t instruction = chunk->code[offset];
    switch (instruction) {
        case op_add:
            return simple_instruction("op_add", offset);
        case op_subtract:
            return simple_instruction("op_subtract", offset);
        case op_multiply:
            return simple_instruction("op_multiply", offset);
        case op_divide:
            return simple_instruction("op_divide", offset);
        case op_constant:
            return constant_instruction("op_constant", chunk, offset);
        case op_constant_long:
            return constant_long_instruction("op_constant_long", chunk, offset);
        case op_negate:                                                         // unary operator(-)
            return simple_instruction("op_negate", offset); 
        case op_return:
            return simple_instruction("op_return", offset);
        default:
            printf("unknown opcode %d\n", instruction);
            return offset + 1;
     }
}


static uint32_t simple_instruction(const char* name, uint32_t offset) {
    printf("%-16s\n", name);
    return offset + 1;
}

static uint32_t constant_instruction(const char* name, Chunk* chunk, uint32_t offset) {
    uint8_t constant_offset = chunk->code[offset + 1];
    printf("%-16s %4d '", name, constant_offset);
    print_value(chunk->constants.values[constant_offset]);
    printf("'\n");
    return offset + 2;
}

static uint32_t constant_long_instruction(const char* name, Chunk* chunk, uint32_t offset) {
    uint16_t constant_offset = (uint16_t)(chunk->code[offset + 1] << 8) | chunk->code[offset + 2];
    printf("%-16s %4d '", name, constant_offset);
    print_value(chunk->constants.values[constant_offset]);
    printf("'\n");
    return offset + 3;
}

