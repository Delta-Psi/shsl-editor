// edi: _arg0
//
// stack size: 0x68
// 0x28: uint64_t _var0
// 0x10: _var1
// 0x08: uint64_t _var2
// 0x58: uint64_t _var3
void _0x575df0(uint32_t _arg0) {
	// 0x575e00
	ebp = _arg0;
	rax = fs:0x28;	// ?????
	_var0 = rax;
	eax = 0;

	// 0x575e10
	rbx = &_var1;
	if (_arg0 == 0) {
		// 0x5763b5...
	}
	if (_arg0 == 1) {
		// 0x576557...
	}
	if (_arg0 == 2) {
		// 0x57657b...
	}

	// 0x575e2f
	edx = 0;
	esi = 0x0a;
	rax = _0x429c10(&_var1); // !! DATA POINTER !!
	_var2 = rax;

	// 0x575e43
	eax = *(uint16_t*)rax;	// first u16 of data

	// 0x575e46
	if (ax == 0) {
		// 0x575e69...
	}

	// 0x575e4b
	edx = *(uint8_t*)0xb3c8c9;
	if (dl != 1) {
		if (dl == 0) {
			// 0x576544...
		}
		if (dl != 2) {
			// 0x576538...
		}
		eax -= 0x3c;
	}

	// 0x575e69
	*(uint16_t*)0xab4a7a = ax;
	edx = ax;
	if (*(uint8_t*)0xb38a4c & 0x10) {
		// 0x575e7c
		ecx = rdx*3;
		edx = 0x66666667;
		ecx <<= 2;	// logical
		eax = ecx;
		ecx >>= 0x1f;	// arithmetic
		edx_eax = edx * eax;	// signed
		edx >>= 2;	// arithmetic
		edx -= ecx;
		*(uint16_t*)0xab4a7a = dx;
	}

	// 0x575e9a
	*(uint16_t*)0xab543c = dx;
	rbx = _var2;	// POINTER TO DATA
	rcx = *(uint64_t*)0xb3e678;
	eax = *(uint16_t*)(rbx + 2);	// second u16 of data

	// 0x575eb1
	*(uint8_t)(rcx + 0x1fc6) = al;
	if (al == 0) {
		// 0x57633f...
	}

	r15d = 0;	// CHUNK COUNTER
	goto _0x576159;

_0x575f08:
	rax = r14 * 0x346;	// signed
	edx = *(uint16_t*)(rbx + 8); // chunk[2]
	edi = (int32_t)-1;
	rax += rcx + 0x22d0;
	*(uint8_t*)(rax + 0x12) = dl;
	edx = *(uint16_t*)(rbx + 10);	// chunk[3]
	if (dx == 0xffff) {
		edx = edi;	// -1
	}

	// 0x575f2e
	*(uint8_t*)(rax + 0x13) = dl;

	edx = *(uint16_t*)(rbx + 12);	// seventh
	*(uint8_t*)(rax + 0x14) = dl;

	eax = *(uint16_t*)(rbx + 14);	// eighth
	rdx = r14 * 0x346;	// signed

	// 0x575f43
	rsi = rcx + rdx + 0x22e0;
	*(uint32_t*)(rsi + 0x0c) = ax;
	if (*(uint8_t*)0xb38a4e == 2) {
		// 0x575f6e...
	}

	// 0x575f58
	edx = ax;
	edi = 0x66666667;
	edx <<= 3; // logical
	eax = edx;
	edx_eax = edi * eax;	// signed
	edx >>= 2; // logical

	*(uint32_t*)(rsi + 0x0c) = dx;
	r13 = r14 * 0x346;	// signed
	edx = *(uint16_t*)(rbx + 0x10);	// ninth
	rcx += r13;
	*(uint8_t*)(rcx + 0x22eb) = dl;

	edx = *(uint16_t*)(rbx + 8); // fifth u16?
	r12 = rcx + 0x22e1;
	*(uint8_t*)(rcx + 0x22ea) = dl;

	// 0x575f93
	esi = *(uint16_t*)(rbx + 0x12);
	*(uint16_t*)(rcx + 0x1fd0) = si;
	// A WHOLE BUNCH OF SHIT LIKE THAT
	// 0x576037
	esi =  *(uint16_t*)(rbx + 0x30); // !
	edi = dl;
	*(uint8_t*)(rcx + 0x22ef) = sil;
	esi = sil;
	_0x589230();

	// 0x57604e
	rcx = *(uint64_t*)0xb3e678;
	*(uint8_t*)(r12 + 0xe) = al; // return value from prev call?
	edx = *(uint16_t*)(rbx + 0x32);	// !
	r8 = rcx + r13;
	rax = r8 + 0x22e0;
	*(uint32_t*)(rax + 0x16) = dx;
	edx = *(uint16_t*)(rbx + 0x34);	// !

	// 0x576071
	*(uint8_t*)(rax + 0x11) = dl;
	edx = *(uint16_t*)(rbx + 0x36);	// !
	*(uint8_t*)(rax + 0x12) = dx;
	if (*(uint8_t*)(r8 + 0x1fc9) != 0) {
		// 0x576190...
	}

	edx = *(uint8_t*)0xb3c8c8;
	if (dl == 1) {
		// 0x576190...
	}
	if (dl < 1) {
		// 0x5762c0...
	}
	if (dl == 2) {
		// 0x5762a8...
	}

	edx = *(uint16_t*)(rax + 0x08);
	if (dx <= 0x3b) {
		rax = r14 * 0x346; // signed
		edx = 0x3c;
		*(uint16_t*)(rcx + rax + 0x22e8) = dx;
	}
	// 0x5760ca
	rdx = r14 * 0x346; // signed
	eax = *(uint16_t*)(rbx + 0x3a); // chunk[27]
	rdx = rcx + rdx + 0x22e0;

	// 0x5760dd
	*(uint16_t*)(rdx + 0x14) = ax;
	if (ax == 0) {	// chunk[27] == 0
		// 0x5760e6
		eax = *(uint16_t*)0xab4840;
		*(uint16_t*)(rdx + 0x14) = ax;
	}

	// 0x5760f1
	rax = r14 * 0x346; // signed
	esi = *(uint16_t*)(rbx + 0x3c);	// chunk[28]
	rax += rcx;

	// 0x5760ff
	rdx = rax + 0x22f0;
	*(uint8_t*)(rdx + 0x08) = sil;
	edi = *(uint16_t*)(rbx + 0x3e);	// chunk[29]
	rsi = rax + 0x22e0;
	*(uint8_t*)(rax + 0x22f0) = dil;
	if (ebp == 0x01) {
		// 0x5761b0...
	}

	// 0x576125
	esi = *(uint8_t*)(rbx + 0x40);	// chunk[30]
	*(uint8_t*)(rdx + 0x0b) = sil;
	edx = *(uint8_t*)(rbx + 0x46);	// chunk[33]
	esi = r15 + 1;
	*(uint8_t*)(rax + 0x22e6) = 0;
	*(uint8_t*)(rax + 0x22e5) = dl;
	edx = *(uint8_t*)(rcx + 0x1fc6);
	
	// 0x576149
	r15 += 1; // increments counter
	rbx += 0x44;	// prolly important, points to 0x44 in the dat
	if (edx <= esi) {
		// 0x576348...
	}

_0x576159:
	// uint16_t* chunk = rbx + 0x4;
	rsi = r15d;	// signed
	eax = *(uint16_t*)(rbx + 0x4);	// chunk[0]
	r14 = r15d;	// signed
	rsi *= 0x346;	// signed
	rsi += rcx;

	// 0x57616d
	*(uint8_t*)(rsi + 0x1fe0) = al;
	if (ebp == 0) {	// ???
		// 0x575ed0...
	}

	// 0x57617b
	eax = *(uint16_t*)(rbx + 0x6);	// chunk[1]
	*(uint8_t*)(rsi + 0x1fc9) = al;
	goto _0x575f08;




_0x57633f:
	edx = 0;
_0x576348:
	eax = *(uint32_t*)0xab4a14;
	*(uint8_t*)(rcx+0xf3cc) = !!(ah & 0x04);
	*(uint8_t*)(rcx+0xf3cd) = !!(ah & 0x08);
	if (ebp == 1) {
		// 0x5763d7...
	}
	rdi = _var2;
	_0x413f50();
	rax = *(uint64_t*)0xb3e678;
	edx = *(uint16_t*)(rax + 0x22f2);
	esx = *(uint16_t*)(rax + 0x22f4);
	edi = *(uint16_t*)(rax + 0x22ee);
	_0x5070d0();
	rax = _var3;
	rax ^= fs_0x28;
	if (rax != 0) {
		// 0x576588...
	}
	return;
}
