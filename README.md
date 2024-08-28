# Description
Following [this project](https://jrdngr.github.io/pngme_book/chapter_1.html) to improve in Rust.  
Obviously using [the Rust Language book](https://doc.rust-lang.org/book/title-page.html) on the side.

# TODO
- [X] Scrambling messages inside chunks list, not only append to end
- [ ] Message in chunk type
   - try and put the message in the chunk types of existing chunks instead of appending to chunks list 
   - should be more discreet in terms of size
   - make the unused chunk types some default values
- [ ] More than one message
    - some way to link messages between them through chunk types?
    - unordered?
- [ ] Message encryption
    - public key through chunk type?
    - some other way to communicate this?
- [ ] Better error handling
   - limit "?" usage and explicit error handling
- [ ] Handle JPEG?
