# Description
Following [this project](https://jrdngr.github.io/pngme_book/chapter_1.html) to improve in Rust.  
Obviously using [the Rust Language book](https://doc.rust-lang.org/book/title-page.html) on the side.

# TODO
- [X] Scrambling messages inside chunks list, not only append to end
- [X] Message in chunk type
   - try and put the message in the chunk types of existing chunks instead of appending to chunks list
   - make the unused chunk types some default values (cannot do it for all, check [critical chunks](http://www.libpng.org/pub/png/spec/1.2/PNG-Chunks.html#C.Critical-chunks))
     - I'd like to do an occurrences count for each chunk types (including the critical ones)
     - does it allow the storage of a message in the chunk type of non-critical chunks?
   - should be more discreet in terms of size
   - SEEMS IT CANNOT BE DONE, AS MANY CHUNKS HAVE A CRITICAL TYPE, WHICH CANNOT BE CHANGED
- [ ] More than one message
    - some way to link messages between them through chunk types?
    - unordered?
- [ ] Message encryption
    - public key through chunk type?
    - some other way to communicate this?
- [ ] Better error handling
   - limit "?" usage and explicit error handling
- [ ] Handle JPEG?
