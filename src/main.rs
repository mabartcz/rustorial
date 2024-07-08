mod structs;
mod option;
mod generic;
mod buffer;
mod vectors;
mod info;
mod hashmaps;
mod iterators;
mod time;
mod threading;
mod scope_threads;
mod mutexs;
mod channels;
mod myflatbuffers;

#[allow(dead_code)]
fn main() {
    // option::test_option();
    // structs::test_structs();
    // generic::test_generic();
    // buffer::test_buffer();
    // vectors::test_vectors();
    // vectors::test_vec_string();
    // info::system_info();
    // hashmaps::test_hashmap();
    // hashmaps::test_hashset();
    // iterators::test_iterators();
    // time::time_test();
    // time::test_chrono()
    // threading::test_threading();
    // scope_threads::test_thread_variables();
    // mutexs::text_mutex();
    channels::test_channels();
}


