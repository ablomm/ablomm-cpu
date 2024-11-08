main:
st.eq r1, other;
ld.ne r2, main;
	push r1;
	int;
	pop r2;
other:

