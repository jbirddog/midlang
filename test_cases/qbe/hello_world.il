data $hello_world_str0 = { b "hello world", b 0 }
export function w $main() {
@start
    %r =w call $puts(l $hello_world_str0)
    ret 0
}
