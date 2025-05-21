from starkware.cairo.common.registers import get_fp_and_pc

// Accepts a pointer called my_tuple.
func foo(my_tuple: felt*) {
    // 'my_tuple' points to the 'numbers' tuple.
    let a = my_tuple[1];  // a = 2.
    return ();
}

func main() {
    alloc_locals;
    // Get the value of the fp register.
    let (__fp__, _) = get_fp_and_pc();
    // Define a tuple.
    local numbers: (felt, felt, felt) = (1, 2, 3);
    // Send the address of the 'numbers' tuple.
    foo(&numbers);
    return ();
}
