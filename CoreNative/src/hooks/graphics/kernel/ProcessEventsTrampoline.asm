PUBLIC ProcessEventsTrampoline
PUBLIC GetRenderContextForThread

EXTERN g_BaseAddress: qword
EXTERN ProcessEventsJumpTable9: PROC

_TEXT	SEGMENT

ProcessEventsTrampoline:
	MOV rcx, r10
	MOV rax, g_BaseAddress
	ADD rax, 30FD84h
	PUSH rax
	JMP ProcessEventsJumpTable9

GetRenderContextForThread:
	PUSH rcx
	PUSH rdx

	; Retrieve the TlsIndex
	MOV rdx, g_BaseAddress
	MOV edx, [rdx+1F37534h]

	; Retrieve the specific slot
	MOV rax, gs:[58h]
	MOV rax, [rax+rdx*8]

	; Retrieve our context from the slot
	MOV rcx, 250h
	MOV rax, [rax+rcx]

	POP rdx
	POP rcx

	RET

_TEXT	ENDS

END