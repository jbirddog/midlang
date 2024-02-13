data $not_str0 = { b "not works!", b 0 }
export function w $main() {
@start
    %..not..1 =w sub 1, 1
    jnz %..not..1, @..cond..0_case_0, @..cond..0_case_0_end
@..cond..0_case_0
    call $exit(w 1)
    jmp @..cond..0_end
@..cond..0_case_0_end
    %..cmp..3 =w cnew 12, 21
    %..not..2 =w sub 1, %..cmp..3
    jnz %..not..2, @..cond..0_case_1, @..cond..0_case_1_end
@..cond..0_case_1
    call $exit(w 2)
    jmp @..cond..0_end
@..cond..0_case_1_end
    %..cmp..5 =w ceql 12, 12
    %..not..4 =w sub 1, %..cmp..5
    jnz %..not..4, @..cond..0_case_2, @..cond..0_case_2_end
@..cond..0_case_2
    call $exit(w 3)
    jmp @..cond..0_end
@..cond..0_case_2_end
@..cond..0_end
    call $puts(l $not_str0)
    ret 0
}
