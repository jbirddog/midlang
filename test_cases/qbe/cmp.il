data $cmp_str0 = { b "cmp works!", b 0 }
export function w $main() {
@start
    %..cmp..0 =w ceqw 0, 1
    jnz %..cmp..0, @cond_case_0, @cond_case_0_end
@cond_case_0
    call $exit(w 1)
    jmp @cond_end
@cond_case_0_end
    %..cmp..1 =w ceqw 12, 21
    jnz %..cmp..1, @cond_case_1, @cond_case_1_end
@cond_case_1
    call $exit(w 2)
    jmp @cond_end
@cond_case_1_end
    %..cmp..2 =w cnel 12, 12
    jnz %..cmp..2, @cond_case_2, @cond_case_2_end
@cond_case_2
    call $exit(w 3)
    jmp @cond_end
@cond_case_2_end
@cond_end
    call $puts(l $cmp_str0)
    ret 0
}
