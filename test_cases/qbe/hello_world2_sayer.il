data $hello_world2_sayer_str0 = { b "hello world", b 0 }
export function w $say_hello_world() {
@start
    %r =w call $puts(l $hello_world2_sayer_str0)
    ret 0
}
