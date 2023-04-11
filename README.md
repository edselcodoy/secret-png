# 🤫🖼️ Secret PNG

A project based on the [PNGme](https://picklenerd.github.io/pngme_book/introduction.html) exercise which interestingly lets one hide secret messages inside PNG files.

Specifically, the command line program can:
1. **Encode** a message into a PNG file
2. **Decode** a message stored in a PNG file
3. **Remove** a message from a PNG file
4. **Print** a list of PNG chunks that can be searched for messages

## Introduction

---

The program follows the [PNG file structure spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html) in which it inspects PNG files in a chunk-level. A single PNG image is composed of a series of chunks. These chunks have types which a decoder can use in determining the property of a chunk. For example, every PNG file begins with an IHDR-type chunk and ends with an IEND-type chunk. 

A valid **chunk type** is a *4-byte* code, wherein each byte can be represented by uppercase or lowercase letters (in UTF-8), e.g. 'tIME', 'bLOb', and 'coOl'. Each of its bytes implies a unique property of the chunk. For instance, an uppercase first byte/letter in the chunk type means that the chunk is "critical" for a successful display of the file's contents, otherwise it is ancillary. The second byte tells the decoder whether the chunk is private or public. Furthermore, the third byte is a reserved for future expansion, while the fourth/last byte serves as a safe-to-copy flag for the chunk. You can read more about this in the PNG file spec.

We can use unique codes like the chunk types to create our own chunk, containing any message we want. Inserting this new ancillary chunk to an input PNG file will yield an image whose display is the same as the original but with some hidden message embedded on it. Utilizing unique chunk types as *keys* will help us encode and decode the message inside the PNG file. 

## Installation

---

You need to have Rust package manager ```cargo``` downloaded in your machine to install this command line program.

Install this directly to your machine using
```bash
    cargo install --git https://github.com/edselcodoy/secret-png.git
```

Then you can now use and call `secret-png` in your machine.

## Commands

---

### 1. Encode message
```
    secret-png encode <file-path> <chunk-type> <message> [output-file]
```
where ```output-file``` is optional (defaults to ```<file-path>```, overwrites it).

Example:
```
    secret-png encode test.png cOOl "this messsage will be hidden" output.png
```

### 2. Decode message
```
    secret-png decode <file-path> <chunk-type>
```

Example:
```
    secret-png decode output.png cOOl
```

### 3. Remove message
```
    secret-png remove <file-path> <chunk-type>
```

Example:
```
    secret-png remove output.png cOOl
```

### 4. Print all chunks inside the PNG file
```
    secret-png print <file-path>
```

Example:
```
    secret-png print output.png
```

---

If there is any parameter confusion, you can ask for help by
```
    secret-png <command> -h
```
where `<command>` may be `encode`, `decode`, `remove`, or `print`.