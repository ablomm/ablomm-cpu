thing = 123;
{
	thing = 321;
	ld r1, [r2 + 6];
	{
		another_thing = 0x123;

	}
	ld r1, [r2-6];
}
