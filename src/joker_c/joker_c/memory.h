#pragma once

#ifndef joker_memory_h
#define joker_memory_h

#include "common.h"

// macro for grow capacity
#define macro_grow_capacity(capacity) ((capacity) < 8 ? 8 : (capacity) * 2)
// macro for grow array
#define macro_grow_array(type, pointer, old_capacity, new_capacity) \
	(type*)reallocate(												\
		pointer,													\
		sizeof(type) *(old_capacity),								\
        sizeof(type) *(new_capacity)								\
	)
// macro for free array
#define macro_free_array(type, pointer, old_capacity)	\
    reallocate(											\
		pointer,										\
		sizeof(type) * (old_capacity),					\
		0												\
	)

/*
* reallocate memory
* 
	oldSize	 new_size	 operator
		0	   !0			Allocate new block.			�����¿�
	   !0		0			Free allocation.			�ͷ��ѷ����ڴ�
	   !0		< oldSize	Shrink existing allocation. �����ѷ����ڴ�
	   !0		> oldSize	Grow existing allocation.	�����ѷ����ڴ�
*/
void* reallocate(void* pointer, size_t old_size, size_t new_size);


/*
#ifndef joker_memory_worker
#define joker_memory_worker

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <string.h>

#define HEAP_SIZE 1024 * 1024  // 1MB�Ķ��ڴ�

typedef struct Block {
    size_t size;       // ��Ĵ�С
    struct Block* next; // ָ����һ�����п�
    int is_free;       // �Ƿ��ͷ�
} Block;

static uint8_t private_heap[HEAP_SIZE]; // ˽�˶��ڴ�
static Block* free_list = NULL; // ���п�����
static size_t used_size = 0; // ��ǰ��ʹ�õ��ڴ��С

// ��ʼ���ڴ����ϵͳ
void init_memory() {
    free_list = (Block*)private_heap; // ���ÿ����б�Ŀ�ͷ
    free_list->size = HEAP_SIZE - sizeof(Block);
    free_list->next = NULL;
    free_list->is_free = 1;
}

// �����ڴ��
void* my_malloc(size_t size) {
    Block* curr = free_list;
    while (curr) {
        if (curr->is_free && curr->size >= size) {
            // �ҵ����ʵĿ�
            curr->is_free = 0; // ���Ϊ�ѷ���
            // ���ʣ��ռ��㹻���ָ��
            if (curr->size >= size + sizeof(Block)) {
                Block* next_block = (Block*)((uint8_t*)curr + sizeof(Block) + size);
                next_block->size = curr->size - size - sizeof(Block);
                next_block->next = curr->next;
                next_block->is_free = 1;

                curr->size = size; // ������ǰ��Ĵ�С
                curr->next = next_block; // ���µ�ǰ���ָ��
            }
            return (void*)((uint8_t*)curr + sizeof(Block)); // ������������ַ
        }
        curr = curr->next; // �ƶ�����һ�����п�
    }
    return NULL; // û���㹻���ڴ�
}

// �ͷ��ڴ��
void my_free(void* ptr) {
    if (!ptr) return; // ��ָ�벻����

    Block* block = (Block*)((uint8_t*)ptr - sizeof(Block));
    block->is_free = 1; // ���Ϊ���ͷ�

    // �ϲ����ڵĿ��п����������ʵ�֣���ѡ��
}

// reallocate ʵ��
void* reallocate(void* ptr, size_t new_size) {
    if (ptr == NULL) {
        return my_malloc(new_size); // ���ԭָ��Ϊ�գ���Ч�� malloc
    }

    if (new_size == 0) {
        my_free(ptr); // ��� new_size Ϊ 0����Ч�� free
        return NULL;
    }

    Block* old_block = (Block*)((uint8_t*)ptr - sizeof(Block));

    // �ж��Ƿ������ԭ�ڴ������������
    if (new_size <= old_block->size) {
        return ptr; // ֱ�ӷ���ԭָ��
    }
    else {
        // ��Ҫ�µ��ڴ��
        void* new_ptr = my_malloc(new_size);
        if (new_ptr) {
            // ��������
            memcpy(new_ptr, ptr, old_block->size);  // ���ƾ�����
            my_free(ptr); // �ͷžɵ��ڴ��
        }
        return new_ptr; // ������ָ��
    }
}

int main() {
    init_memory(); // ��ʼ���ڴ����
    // ʾ���÷�
    int* arr = (int*)reallocate(NULL, 10 * sizeof(int));
    for (int i = 0; i < 10; i++) {
        arr[i] = i;
    }

    arr = (int*)reallocate(arr, 20 * sizeof(int)); // ��չ��С
    for (int i = 10; i < 20; i++) {
        arr[i] = i;
    }

    my_free(arr); // �ͷ��ڴ�

    return 0;
}


#endif / * joker_memory_worker * /
*/


#endif /* joker_memory_h */

