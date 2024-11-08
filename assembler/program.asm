main:
st.eq r1, other;
ld.ne r2, [main];
st.ne r2, [other];
	push r1;
	int;
	pop r2;
	add r1, r2;
	sub r1, 123, r1;
	sub r1, r1;
other:

