use cxx::let_cxx_string;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        // 引入C++头文件，使其中的定义（如类型、函数等）可在Rust中使用。
        include!("cxx/cpp/include/blobstore.h");
 
        // 声明一个C++中定义的类型BlobstoreClient在Rust中的别名。
        // 注意：这里并没有真正地在Rust中创建一个新的类型，而是声明了一个与C++中BlobstoreClient类型对应的Rust别名。
        // 这个别名使得Rust代码可以引用和操作C++中的BlobstoreClient类型。
        type BlobstoreClient;
 
        // 声明一个C++中定义的函数new_blobstore_client在Rust中的原型。
        // 这个函数返回一个指向BlobstoreClient类型的唯一指针（UniquePtr）。
        // UniquePtr是C++中的一种智能指针，用于管理动态分配的内存，确保在不再需要时自动释放。
        // 在Rust中，这个UniquePtr类型需要被适当地封装和转换，以便与Rust的内存管理机制兼容。
        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;

        // 'a
        fn get<'a>(self: &'a BlobstoreClient,  str: &'a CxxString) -> &'a CxxString;
    }
}

fn main() {

  
    let client = ffi::new_blobstore_client();

    let_cxx_string!(key = "helloworld");

    println!("client.get=>{}", client.get(&key));
}