data $frexp_str0 = { b "frexp(2560.0, &e); e = %d\n", b 0 }
export function w $main() {
@start
    %exp =w copy 0
    %..ref..0 =l alloc8 8
    storew %exp, %..ref..0
    call $frexp(d d_2560, l %..ref..0)
    %exp =w loadw %..ref..0
    call $printf(l $frexp_str0, w %exp)
    ret 0
}
