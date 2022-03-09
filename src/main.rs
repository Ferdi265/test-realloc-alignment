#![deny(unsafe_op_in_unsafe_fn)]

mod dlmalloc;

#[global_allocator]
static GLOBAL: dlmalloc::DLMalloc = dlmalloc::DLMalloc;

#[repr(C, align(128))]
#[derive(Clone, Copy)]
struct Big([u8; 128]);

impl Default for Big {
    fn default() -> Big {
        Big([0; 128])
    }
}

fn is_aligned<T>(t: *const T) -> bool {
    let align = std::mem::align_of::<T>();
    let ptr = t as usize;

    ptr & (align - 1) == 0
}

fn main() {
    let mut v: Vec<Big> = Vec::new();

    println!(">> allocating vector");
    // allocate aligned chunk
    v.resize(4, Big::default());
    println!(">> allocating vector = {:?}", v.as_slice().as_ptr());

    if !is_aligned(v.as_slice().as_ptr()) {
        panic!("error: vector is not aligned");
    }

    // allocate unrelated junk chunk
    println!(">> allocating junk");
    let _j = Box::new(Big::default());

    // reallocate aligned chunk
    println!(">> reallocating vector");
    v.resize(8, Big::default());
    println!(">> reallocating vector = {:?}", v.as_slice().as_ptr());

    if !is_aligned(v.as_slice().as_ptr()) {
        panic!("error: vector is not aligned after realloc");
    }
}
