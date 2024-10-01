/*
	The bytecode virtual machine.
	This file defines the Value type used in the virtual machine.

	1. �������̶���С��ֵ�� -(ָ�) 
		-> immediate instructions( ��ʱָ��[ ���롢��䡢�ֽ�˳�� ] ): [opcode + immediate value ]
		
		ֵ�ı���λ������ָ����֮��
	2. �ַ��� (�ɱ��С��ֵ) -�������ƿ�ִ���ļ���-(����������)
		-> instuctions( ����ָ�� )�� [opcode + constant index]

		��ַ������������ָ����֮��

	---------------------------------------
	������( Constant Pool )��
		����ַ����������ַ��������ĵ�ַ�������ڶ����ƿ�ִ���ļ��С�
		��������һ��ֵ�����顣[arrary[0], arrary[1],... arrary[n]
		���س�����ָ���������������ȡֵ��[index]
	---------------------------------------

	instruction format( ָ���ʽ ): 
		[opcode + operand1 + operand2 + ... ]

		ÿ��������ᶨ�����ж��ٲ������Լ����Եĺ���
*/

#pragma once

#ifndef joker_value_h
#define joker_value_h

#include "common.h"

typedef double Value;

// ֵ��������
typedef struct {
	uint32_t count;
	uint32_t capacity;
	Value* values;
} ValueArray;

void init_value_array(ValueArray* array);
void free_value_array(ValueArray* array);
void write_value_array(ValueArray* array, Value value);


void print_value(Value value);



#endif /* joker_value_h */

