#pragma once

#ifndef JOKER_CHUNK_H
#define JOKER_CHUNK_H

#include "common.h"
#include "value.h"

typedef uint32_t index_t;
typedef uint32_t line_t;

/*
 * OpCode: ������
 */
typedef enum
{
	op_return,		  //  8 bit
	op_constant,	  //  8 bit
	op_constant_long, // 24 bit
	op_add,			  //  8 bit(+)
	op_subtract,	  //  8 bit(-)
	op_multiply,	  //  8 bit(*)
	op_divide,		  //  8 bit(/)
	op_negate,		  //  8 bit(unary(-))
} OpCode;

typedef struct
{
	line_t line;
	uint32_t count;
} RleLine;

typedef struct
{
	uint32_t count;
	uint32_t capacity;
	RleLine *lines;
} RleLines;
void init_rle_lines(RleLines *lines);
void free_rle_lines(RleLines *lines);
line_t get_rle_line(RleLines *lines, index_t code_count);

/*
 * Chunk: �ֽ����
 *  - count: ָ������
 *  - capacity: ָ������
 *  - code: ָ������
 *
 *	ָ��������ÿ��Ԫ�ص�����Ϊ uint8_t, ��ʾһ���ֽڵ�ָ��.
 */
typedef struct
{
	uint32_t count;
	uint32_t capacity;
	uint8_t *code;		  // ָ������ (������ | ������)
	RleLines lines;		  // �к�����
	ValueArray constants; // ��������
} Chunk;
void init_chunk(Chunk *chunk);
void free_chunk(Chunk *chunk);
void write_chunk(Chunk *chunk, uint8_t code, line_t line);

void write_constant(Chunk *chunk, Value value, line_t line);

#endif /* joker_chunk_h */
