Following [this project](https://jrdngr.github.io/pngme_book/chapter_1.html) to improve in Rust.  
Obviously using [the Rust Language book](https://doc.rust-lang.org/book/title-page.html) on the side.

Should try and put the message in the chunk_type instead of the data.
Make it so that you can have more than one chunk for a message, and find all of them when decoding. 
    (With additional info, like indices?)
Make the add chunk append somewhere in the middle