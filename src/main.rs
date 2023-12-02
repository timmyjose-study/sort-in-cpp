#[cxx::bridge(namespace = "org::sorting")]
mod ffi {
    unsafe extern "C++" {
        include!("sort-in-cpp/include/sort.h");

        type Sorter;

        fn new_sorter() -> UniquePtr<Sorter>;
        fn sort(&self, numbers: &mut Vec<i32>);
    }
}

fn main() {
    let client = ffi::new_sorter();
    let mut numbers = vec![5, 1, 4, 2, 3];
    println!("Before sorting: {numbers:#?}");
    client.sort(&mut numbers);
    println!("After sorting: {numbers:#?}");
}
