func main(){
    alloc_locals;
    local x = 10;
    local y = 20;
    local z = add(x, 3);
    local w = add(y, 4);
    local v = z*w;
    ret;
}

func add(a, b){
    alloc_locals;
    local c = a+b;
    return c;
}