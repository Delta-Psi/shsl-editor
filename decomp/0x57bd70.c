// _arg0: edi		<- called w/ *0xab4858
// _arg1: esi (flags?)	<- called w/ 0x20
// _arg2: edx		<- called w/ 0
//
// stack space: 0x38
// _var0: dword 0x18
// _var1: dword 0x0c
// _var2: dword 0x24
// _var3: dword 0x20
// _var4: dword 0x1c
// _var5: dword 0x10
//
// globals:
// *(0xb3e678 + 0x60) is a pointer to the lin data
// 0xab4858: lin instruction pointer??
void _0x57bd70(uint32_t _arg0, uint32_t _arg1, uint32_t _arg2) {
	uint8_t *argument_length_table = 0xa72980;

	r15d = _arg1;
	r14d = _arg1;
	ebx = _arg0;
	r15d &= 0x20; // _arg1 & 0x20
	rax = *(uint64_t*)0xb3e678;
	_var0 = _arg2;
	if (r15d != 0) { // _arg1 & 0x20
		edx = 0;
		*(uint16_t)(rax + 0xfc7e) = dx; // 0?
	}
	// 0x57bda0
	esi = *(uint32_t)0xab4a54;
	r8d = r14d; // _arg1
	r9d = r14d; // _arg1
	r11d = r14d; // _arg1
	r10d = r14d; // _arg1
	r13d = 0;

	r8d &= 0x10; // _arg1 & 0x10
	r9d &= 0x01; // _arg1 & 0x01
	r11d &= 0x02; // _arg1 & 0x02

	_var1 = esi;
	_var1 >>= 9;

	r10d &= 0x08; // _arg1 & 0x08
	_var1 &= 0x01;

	// 0x57bdd8
	rcx = *(uint64_t*)(rax+0x60); // <- POINTER TO LIN DATA!!
	loop {
		// 0x57bde0
		edx = bx; // _arg0, lower 16 bytes
		ebx += 0x02; // _arg0 + 0x02

		eax = *(uint8_t)(rcx + rdx);		// 0x70
		edx = *(uint8_t)(rcx + rdx + 1);	// opcode
		eax <<= 8;
		eax += edx;

		// 0x57bdf4
		r12d = rax - 0x7000;
		if (r12d > 0x4d) {
			// goto 0x57bde0
			continue;
		}

		ebp = eax;
		eax = rax - 0x701a;
		if ((uint16_t)eax <= 0x02) { // 0x1a <= opcode <= 0x1c
			// goto 0x57bfe0
			return;
		}
		if (ebp == 0x7032) {	// 0x32
			// goto 0x57bfe0
			return;
		}
		if (ebp == 0x7035) {	// 0x35
			// goto 0x57bfe0
			return;
		}

		eax = ebp;
		eax &= 0xffffffef; // opcode & 0b1110_1111?
		if (eax == 0x702b) {	// 0x2b, 0x3b
			// goto 0x57bfe0
			return;
		}

		if (ebp == 0x7005) {	// 0x05
			// goto 0x57bfe0
			return;
		}
		if (ebp == 0x704d) {	// 0x4d
			// goto 0x57bfe0
			return;
		}

		eax = rbp - 0x7046;
		if ((uint16_t)eax <= 0x04) {	// 0x46 <= op <= 0x4a
			// goto 0x57bfe0
			return;
		}

		if (ebp == 0x7025) { // 0x25: CHANGE UI!!!
			// goto 0x57bfb0
			edx = bx;
			eax = *(uint8_t)(rcx + rdx); // element?
			if (al == 0x9) {
				// goto 0x57bfef
			}
			if (al == 0x10) {
				// goto 0x57bfe0
				return;
			}

			edx = rax - 0x12;
			if (dl <= 0x02) {
				// goto 0x57bfe0
				return;
			}

			edx = rax - 0x1f;
			if (dl <= 0x04) {
				// goto 0x57bfe0
				return;
			}

			if (al == 0x25) {
				return;
			}
		}
		// 0x57be6d
		if ((r15d != 0) && !(r13b & 0x20)) {
			if (ebp == 0x7008) {	// 0x08
				// 0x57c2a4
			}
		}

		// 0x57be84
		if ((r8d != 0) && !(r13b & 0x10)) {
			if (ebp == 0x701e) {
				// 0x57bffd
			}
			if (ebp == 0x7021) {
				rcx = *(uint64_t*)0xb3e678;
				eax = bx;
				rdx = *(uint64_t*)(rcx+0x60);
				eax = *(uint8_t*)(rdx+rax);

				if (al == 0x3e) {
					// 0x57bf30
				}
				if (al == 0x3f) {
					// 0x57c47c
				}
				*(uint8_t*)0xab4aa4 = al;
				edx = al;
				_var2 = r10d;
				_var3 = r11d;
				_var4 = r9d;
				_var5 = r8d;
				if (*(uint8_t)0xab4a5a == 0x8) {
					// 0x57c438
				}

				edi = *(uint16_t)(rcx+0xf818);
				esi = 0x7a4455;
				_0x427e40();

				r8d = _var5;
				r9d = _var4;
				r11d = _var3;
				r10d = _var2;
				
				// 0x57bf23
				r13d ||= 0x10;
			}

			if (ebp == 0x7002) {
				// 0x57bf23
				r13d ||= 0x10;
			}
		}

		// 0x57bf30
		if ((r9d != 0) && !(r13b & 0x01)) {
			if (ebp == 0x701e) {
				// 0x57c08b
			}
		}

		// 0x57bf50
		if (r11d != 0 && !(r13b & 0x02)) {
			if (ebp == 0x7006) {
				// 0x57c10a
			}
		}

		// 0x57bf70
		if (r10d != 0 && !(r13b & 0x08)) {
			if (ebp == 0x7015) {	// 0x15
				// 0x57c075
				if (!(*(uint8_t*)0xab4a54 & 0x80)) {
					// 0x57c218
					rax = *(uint64_t*)0xb3e678;
					ecx = *(uint64_t*)0xab4858;
					rax = *(uint64_t*)(rax+0x60);
					edx = ecx;
					ecx += 1;
					edx = *(uint8_t*)(rax + rdx);
					ebp = *(uint8_t*)(rax + rcx);
					// TODO
				}
				// 0x57c082
				r13d |= 0x08;
			}
		}

		// 0x57bf90
		if (r14d == r13d) {
			// 0x57bfe0
		}

		r12 = r12d;
		eax = argument_length_table[r12];
		ebx += eax;
		rax = *(uint64_t*)0xb3e678;
		// goto 0x57bdd8
	}
}
