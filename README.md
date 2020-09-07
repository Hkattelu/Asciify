[<img alt="github" src="https://img.shields.io/badge/github-hkattelu/Asciify-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/hkattelu/Asciify)
[<img alt="crates.io" src="https://img.shields.io/crates/v/asciify.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/asciify)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-asciify-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/asciify)

# Asciify
A rust library for converting images to a readable format on the command line

You can import the library straight from this git repo by adding 

```
asciify = "0.1.4"
```

to your `Cargo.toml`.

To use the code, simply construct a builder and set the properties desired.
You can then choose to print the text straight to the console, or convert
it to a string for later use:

```rust
// Printing to console with color
AsciiBuilder::new_from_path(opt.input)
    .set_deep(false);
    .set_invert(false);
    .to_std_out(true);
```

![Ascii chocobo](https://raw.githubusercontent.com/Hkattelu/Asciify/d719722f3e68c13f13782dcb0f67cb75d889a8dd/images/ascii-chocobo.PNG)

```rust
// Generating a string and resizing
AsciiBuilder::new_from_path(opt.input)
    .set_deep(true);
    .set_resize(Some(32, 32));
    .build();
```

![Ascii chocobo after resizing](https://raw.githubusercontent.com/Hkattelu/Asciify/d719722f3e68c13f13782dcb0f67cb75d889a8dd/images/ascii-chocobo-resized.PNG)


```shell
$ ./asciify ./test-images/SPECIAL(ChocoboA)900.png -c
```

![Ascii chocobo with color](https://raw.githubusercontent.com/Hkattelu/Asciify/d719722f3e68c13f13782dcb0f67cb75d889a8dd/images/ascii-chocobo-color.PNG)
