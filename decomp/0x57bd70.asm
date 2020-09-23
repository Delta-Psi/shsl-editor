│   0x57bd70    push   r15
│   0x57bd72    mov    r15d,esi                                                                                                                                                                 │
│   0x57bd75    push   r14                                                                                                                                                                      │
│   0x57bd77    mov    r14d,esi                                                                                                                                                                 │
│   0x57bd7a    push   r13                                                                                                                                                                      │
│   0x57bd7c    push   r12                                                                                                                                                                      │
│   0x57bd7e    push   rbp                                                                                                                                                                      │
│   0x57bd7f    push   rbx                                                                                                                                                                      │
│   0x57bd80    mov    ebx,edi                                                                                                                                                                  │
│   0x57bd82    sub    rsp,0x38                                                                                                                                                                 │
│   0x57bd86    and    r15d,0x20                                                                                                                                                                │
│   0x57bd8a    mov    rax,QWORD PTR [rip+0x5c28e7]        # 0xb3e678                                                                                                                           │
│   0x57bd91    mov    DWORD PTR [rsp+0x18],edx                                                                                                                                                 │
│   0x57bd95    je     0x57bda0                                                                                                                                                                 │
│   0x57bd97    xor    edx,edx                                                                                                                                                                  │
│   0x57bd99    mov    WORD PTR [rax+0xfc7e],dx                                                                                                                                                 │
│  >0x57bda0    mov    esi,DWORD PTR [rip+0x538cae]        # 0xab4a54                                                                                                                           │
│   0x57bda6    mov    r8d,r14d                                                                                                                                                                 │
│   0x57bda9    mov    r9d,r14d                                                                                                                                                                 │
│   0x57bdac    mov    r11d,r14d                                                                                                                                                                │
│   0x57bdaf    mov    r10d,r14d                                                                                                                                                                │
│   0x57bdb2    xor    r13d,r13d                                                                                                                                                                │
│   0x57bdb5    and    r8d,0x10                                                                                                                                                                 │
│   0x57bdb9    and    r9d,0x1                                                                                                                                                                  │
│   0x57bdbd    and    r11d,0x2                                                                                                                                                                 │
│   0x57bdc1    mov    DWORD PTR [rsp+0xc],esi                                                                                                                                                  │
│   0x57bdc5    shr    DWORD PTR [rsp+0xc],0x9                                                                                                                                                  │
│   0x57bdca    and    r10d,0x8                                                                                                                                                                 │
│   0x57bdce    and    DWORD PTR [rsp+0xc],0x1                                                                                                                                                  │
│   0x57bdd3    nop    DWORD PTR [rax+rax*1+0x0]                                                                                                                                                │
│ l 0x57bdd8    mov    rcx,QWORD PTR [rax+0x60]                                                                                                                                                 │
│   0x57bddc    nop    DWORD PTR [rax+0x0]                                                                                                                                                      
│  >0x57bde0    movzx  edx,bx                                                                                                                                                                   │
│   0x57bde3    add    ebx,0x2                                                                                                                                                                  │
│ R 0x57bde6    movzx  eax,BYTE PTR [rcx+rdx*1]                                                                                                                                                 │
│   0x57bdea    movzx  edx,BYTE PTR [rcx+rdx*1+0x1]                                                                                                                                             │
│   0x57bdef    shl    eax,0x8                                                                                                                                                                  │
│   0x57bdf2    add    eax,edx                                                                                                                                                                  │
│   0x57bdf4    lea    r12d,[rax-0x7000]                                                                                                                                                        │
│   0x57bdfb    cmp    r12d,0x4d                                                                                                                                                                │
│   0x57bdff    ja     0x57bde0                                                                                                                                                                 │
│   0x57be01    mov    ebp,eax                                                                                                                                                                  │
│   0x57be03    lea    eax,[rax-0x701a]                                                                                                                                                         │
x   0x57be09    cmp    eax,0x2                                                                                                                                                                  │
x   0x57be0c    jbe    0x57bfe0                                                                                                                                                                 │
x   0x57be12    cmp    ebp,0x7032                                                                                                                                                               │
x   0x57be18    je     0x57bfe0                                                                                                                                                                 │
x   0x57be1e    cmp    ebp,0x7035                                                                                                                                                               │
x   0x57be24    je     0x57bfe0                                                                                                                                                                 │
x   0x57be2a    mov    eax,ebp                                                                                                                                                                  │
x   0x57be2c    and    eax,0xffffffef                                                                                                                                                           │
x   0x57be2f    cmp    eax,0x702b                                                                                                                                                               │
x   0x57be34    je     0x57bfe0                                                                                                                                                                 │
x   0x57be3a    cmp    ebp,0x7005                                                                                                                                                               │
x   0x57be40    je     0x57bfe0                                                                                                                                                                 │
x   0x57be46    cmp    ebp,0x704d                                                                                                                                                               │
x   0x57be4c    je     0x57bfe0                                                                                                                                                                 │
x   0x57be52    lea    eax,[rbp-0x7046]                                                                                                                                                         │
x   0x57be58    cmp    eax,0x4                                                                                                                                                                  │
x   0x57be5b    jbe    0x57bfe0                                                                                                                                                                 │
x   0x57be61    cmp    ebp,0x7025                                                                                                                                                               │
x   0x57be67    je     0x57bfb0                                                                                                                                                                 │
x  >0x57be6d    test   r15d,r15d                                                                                                                                                                │
x   0x57be70    je     0x57be84                                                                                                                                                                 │
x   0x57be72    test   r13b,0x20                                                                                                                                                                │
x   0x57be76    jne    0x57be84                                                                                                                                                                 │
x   0x57be78    cmp    ebp,0x7008                                                                                                                                                               │
x   0x57be7e    je     0x57c2a4                                                                                                                                                                 │
x  >0x57be84    test   r8d,r8d                                                                                                                                                                  │
x   0x57be87    je     0x57bf30                                                                                                                                                                 │
x   0x57be8d    test   r13b,0x10                                                                                                                                                                │
x   0x57be91    jne    0x57bf30                                                                                                                                                                 │
x   0x57be97    cmp    ebp,0x7002                                                                                                                                                               │
x   0x57be9d    je     0x57bf23                                                                                                                                                                 │
x   0x57bea3    cmp    ebp,0x701e                                                                                                                                                               │
x   0x57bea9    je     0x57bffd                                                                                                                                                                 │
x   0x57beaf    cmp    ebp,0x7021                                                                                                                                                               │
x   0x57beb5    jne    0x57bf30                                                                                                                                                                 │
x   0x57beb7    mov    rcx,QWORD PTR [rip+0x5c27ba]        # 0xb3e678                                                                                                                           │
x   0x57bebe    movzx  eax,bx                                                                                                                                                                   │
x   0x57bec1    mov    rdx,QWORD PTR [rcx+0x60]                                                                                                                                                 │
x   0x57bec5    movzx  eax,BYTE PTR [rdx+rax*1]                                                                                                                                                 │
x   0x57bec9    cmp    al,0x3e                                                                                                                                                                  │
x   0x57becb    je     0x57bf30                                                                                                                                                                 │
x   0x57becd    cmp    al,0x3f                                                                                                                                                                  │
x   0x57becf    je     0x57c47c                                                                                                                                                                 │
x   0x57bed5    mov    BYTE PTR [rip+0x538bc9],al        # 0xab4aa4                                                                                                                             │
x   0x57bedb    movzx  edx,al                                                                                                                                                                   │
x   0x57bede    test   BYTE PTR [rip+0x538b75],0x8        # 0xab4a5a                                                                                                                            │
x   0x57bee5    mov    DWORD PTR [rsp+0x24],r10d                                                                                                                                                │
x   0x57beea    mov    DWORD PTR [rsp+0x20],r11d                                                                                                                                                │
x   0x57beef    mov    DWORD PTR [rsp+0x1c],r9d                                                                                                                                                 │
x   0x57bef4    mov    DWORD PTR [rsp+0x10],r8d                                                                                                                                                 │
x   0x57bef9    je     0x57c438                                                                                                                                                                 │
x   0x57beff    mov    edi,DWORD PTR [rcx+0xf818]                                                                                                                                               │
x   0x57bf05    mov    esi,0x7a4455                                                                                                                                                             │
x   0x57bf0a    call   0x427e40                                                                                                                                                                 │
x   0x57bf0f    mov    r8d,DWORD PTR [rsp+0x10]                                                                                                                                                 │
x   0x57bf14    mov    r9d,DWORD PTR [rsp+0x1c]                                                                                                                                                 │
x   0x57bf19    mov    r11d,DWORD PTR [rsp+0x20]                                                                                                                                                │
x   0x57bf1e    mov    r10d,DWORD PTR [rsp+0x24]                                                                                                                                                │
x  >0x57bf23    or     r13d,0x10                                                                                                                                                                │
x   0x57bf27    nop    WORD PTR [rax+rax*1+0x0]                                                                                                                                                 │
x  >0x57bf30    test   r9d,r9d                                                                                                                                                                  │
x   0x57bf33    je     0x57bf50                                                                                                                                                                 │
x   0x57bf35    test   r13b,0x1                                                                                                                                                                 │
x   0x57bf39    jne    0x57bf50                                                                                                                                                                 │
x   0x57bf3b    cmp    ebp,0x701e                                                                                                                                                               │
x   0x57bf41    je     0x57c08b                                                                                                                                                                 │
x   0x57bf47    nop    WORD PTR [rax+rax*1+0x0]                                                                                                                                                 │
x  >0x57bf50    test   r11d,r11d                                                                                                                                                                │
x   0x57bf53    je     0x57bf70                                                                                                                                                                 │
x   0x57bf55    test   r13b,0x2                                                                                                                                                                 │
x   0x57bf59    jne    0x57bf70                                                                                                                                                                 │
x   0x57bf5b    cmp    ebp,0x7006                                                                                                                                                               │
x   0x57bf61    je     0x57c10a                                                                                                                                                                 │
x   0x57bf67    nop    WORD PTR [rax+rax*1+0x0]                                                                                                                                                 │
x  >0x57bf70    test   r10d,r10d                                                                                                                                                                │
x   0x57bf73    je     0x57bf90                                                                                                                                                                 │
x   0x57bf75    test   r13b,0x8                                                                                                                                                                 │
x   0x57bf79    jne    0x57bf90                                                                                                                                                                 │
x   0x57bf7b    cmp    ebp,0x7015                                                                                                                                                               │
x   0x57bf81    je     0x57c075                                                                                                                                                                 │
x   0x57bf87    nop    WORD PTR [rax+rax*1+0x0]                                                                                                                                                 │
x  >0x57bf90    cmp    r14d,r13d                                                                                                                                                                │
x   0x57bf93    je     0x57bfe0                                                                                                                                                                 │
x   0x57bf95    movsxd r12,r12d                                                                                                                                                                 │
x   0x57bf98    movzx  eax,BYTE PTR [r12+0xa72980]                                                                                                                                              │
x   0x57bfa1    add    ebx,eax                                                                                                                                                                  │
x   0x57bfa3    mov    rax,QWORD PTR [rip+0x5c26ce]        # 0xb3e678                                                                                                                           │
x   0x57bfaa    jmp    0x57bdd8                                                                                                                                                                 │

x  >0x57bfb0    movzx  edx,bx                                                                                                                                                                   │
x   0x57bfb3    movzx  eax,BYTE PTR [rcx+rdx*1]                                                                                                                                                 │
x   0x57bfb7    cmp    al,0x9                                                                                                                                                                   │
x   0x57bfb9    je     0x57bfef                                                                                                                                                                 │
x   0x57bfbb    cmp    al,0x10                                                                                                                                                                  │
x   0x57bfbd    je     0x57bfe0                                                                                                                                                                 │
x   0x57bfbf    lea    edx,[rax-0x12]                                                                                                                                                           │
x   0x57bfc2    cmp    dl,0x2                                                                                                                                                                   │
x   0x57bfc5    jbe    0x57bfe0                                                                                                                                                                 │
x   0x57bfc7    lea    edx,[rax-0x1f]                                                                                                                                                           │
x   0x57bfca    cmp    dl,0x4                                                                                                                                                                   │
x   0x57bfcd    jbe    0x57bfe0                                                                                                                                                                 │
x   0x57bfcf    cmp    al,0x25                                                                                                                                                                  │
x   0x57bfd1    jne    0x57be6d                                                                                                                                                                 │
x   0x57bfd7    nop    WORD PTR [rax+rax*1+0x0]                                                                                                                                                 │
x  >0x57bfe0    add    rsp,0x38                                                                                                                                                                 │
x   0x57bfe4    pop    rbx                                                                                                                                                                      │
x   0x57bfe5    pop    rbp                                                                                                                                                                      │
x   0x57bfe6    pop    r12                                                                                                                                                                      │
x   0x57bfe8    pop    r13                                                                                                                                                                      │
x   0x57bfea    pop    r14                                                                                                                                                                      │
x   0x57bfec    pop    r15                                                                                                                                                                      │
x   0x57bfee    ret                                                                                                                                                                             

x   0x57c075    test   BYTE PTR [rip+0x5389d8],0x80        # 0xab4a54                                                                                                                           │
x   0x57c07c    je     0x57c218                                                                                                                                                                 │
x > 0x57c082    or     r13d,0x8                                                                                                                                                                 │
x   0x57c086    jmp    0x57bf90                                                                                                                                                                 

│   0x57c218    mov    rax,QWORD PTR [rip+0x5c2459]        # 0xb3e678                                                                                                                           │
│   0x57c21f    mov    ecx,DWORD PTR [rip+0x538633]        # 0xab4858                                                                                                                           │
│   0x57c225    mov    rax,QWORD PTR [rax+0x60]                                                                                                                                                 │
│   0x57c229    mov    edx,ecx                                                                                                                                                                  │
│   0x57c22b    add    ecx,0x1                                                                                                                                                                  │
│   0x57c22e    movzx  edx,BYTE PTR [rax+rdx*1]                                                                                                                                                 │
│   0x57c232    movzx  ebp,BYTE PTR [rax+rcx*1]                                                                                                                                                 │
│   0x57c236    shl    edx,0x8                                                                                                                                                                  │
│   0x57c239    add    ebp,edx                                                                                                                                                                  │
│   0x57c23b    cmp    ebp,0xffff                                                                                                                                                               │
│   0x57c241    je     0x57c082                                                                                                                                                                 │
│   0x57c247    movzx  edx,bx                                                                                                                                                                   │
│   0x57c24a    cmp    BYTE PTR [rax+rdx*1+0x2],0x1                                                                                                                                             │
│   0x57c24f    jne    0x57c082                                                                                                                                                                 │
│   0x57c255    movzx  eax,WORD PTR [rip+0x538796]        # 0xab49f2                                                                                                                            │
│   0x57c25c    cmp    eax,ebp                                                                                                                                                                  │
│   0x57c25e    je     0x57c082                                                                                                                                                                 │
│   0x57c264    mov    DWORD PTR [rsp+0x24],r10d                                                                                                                                                │
│   0x57c269    mov    DWORD PTR [rsp+0x20],r11d                                                                                                                                                │
│   0x57c26e    mov    DWORD PTR [rsp+0x1c],r9d                                                                                                                                                 │
│   0x57c273    mov    DWORD PTR [rsp+0x10],r8d                                                                                                                                                 │
│   0x57c278    call   0x589350                                                                                                                                                                 │
│   0x57c27d    mov    edi,ebp                                                                                                                                                                  │
│   0x57c27f    mov    WORD PTR [rip+0x53876c],bp        # 0xab49f2                                                                                                                             │
│   0x57c286    call   0x4cb000                                                                                                                                                                 │
│   0x57c28b    mov    r10d,DWORD PTR [rsp+0x24]                                                                                                                                                │
│   0x57c290    mov    r11d,DWORD PTR [rsp+0x20]                                                                                                                                                │
│   0x57c295    mov    r9d,DWORD PTR [rsp+0x1c]                                                                                                                                                 │
│   0x57c29a    mov    r8d,DWORD PTR [rsp+0x10]                                                                                                                                                 │
│   0x57c29f    jmp    0x57c082                                                                                                                                                                 │
