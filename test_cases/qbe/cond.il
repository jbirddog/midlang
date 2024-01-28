data $cond_str0 = { b "cond", b 0 }
data $cond_str2 = { b "ok", b 0 }
data $cond_str1 = { b "works", b 0 }
export function w $main() {
@start
    jnz 0, @..cond..0_case_0, @..cond..0_case_0_end
@..cond..0_case_0
    jnz 1, @..cond..1_case_0, @..cond..1_case_0_end
@..cond..1_case_0
    call $exit(w 1)
    jmp @..cond..1_end
@..cond..1_case_0_end
@..cond..1_end
    jmp @..cond..0_end
@..cond..0_case_0_end
    jnz 1, @..cond..0_case_1, @..cond..0_case_1_end
@..cond..0_case_1
    %r =w call $puts(l $cond_str0)
    jmp @..cond..0_end
@..cond..0_case_1_end
@..cond..0_end
    jnz 1, @..cond..2_case_0, @..cond..2_case_0_end
@..cond..2_case_0
    %r =w call $puts(l $cond_str1)
    jmp @..cond..2_end
@..cond..2_case_0_end
    jnz 1, @..cond..2_case_1, @..cond..2_case_1_end
@..cond..2_case_1
    jnz 1, @..cond..3_case_0, @..cond..3_case_0_end
@..cond..3_case_0
    call $exit(w 1)
    jmp @..cond..3_end
@..cond..3_case_0_end
@..cond..3_end
    jmp @..cond..2_end
@..cond..2_case_1_end
@..cond..2_end
    call $puts(l $cond_str2)
    ret 0
}
