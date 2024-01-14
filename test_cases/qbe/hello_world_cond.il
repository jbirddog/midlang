data $hello_world_cond_str0 = { b "hello world", b 0 }
export function w $main() {
@start
    jnz 0, @cond_case_0, @cond_case_0_end
@cond_case_0
    call $exit(w 1)
    jmp @cond_end
@cond_case_0_end
    jnz 1, @cond_case_1, @cond_case_1_end
@cond_case_1
    %r =w call $puts(l $hello_world_cond_str0)
    jmp @cond_end
@cond_case_1_end
@cond_end
    ret 0
}
