data $fabs_str0 = { b "The fabs of -1.23 is %f\n", b 0 }
export function w $main() {
@start
    %..arg..0 =d call $fabs(d d_-1.23)
    call $printf(l $fabs_str0, d %..arg..0)
    ret 0
}
