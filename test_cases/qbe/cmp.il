data $cmp_str0 = { b "cmp works!", b 0 }
export function w $main() {
@start
    %..cmp..1 =w ceqw 0, 1
    jnz %..cmp..1, @..cond..0_case_0, @..cond..0_case_0_end
@..cond..0_case_0
    call $exit(w 1)
    jmp @..cond..0_end
@..cond..0_case_0_end
    %..cmp..2 =w ceqw 12, 21
    jnz %..cmp..2, @..cond..0_case_1, @..cond..0_case_1_end
@..cond..0_case_1
    call $exit(w 2)
    jmp @..cond..0_end
@..cond..0_case_1_end
    %..cmp..3 =w cnel 12, 12
    jnz %..cmp..3, @..cond..0_case_2, @..cond..0_case_2_end
@..cond..0_case_2
    call $exit(w 3)
    jmp @..cond..0_end
@..cond..0_case_2_end
@..cond..0_end
    call $puts(l $cmp_str0)
    ret 0
}
