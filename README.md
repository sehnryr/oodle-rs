# oodle-rs

[![GitHub repo size](https://img.shields.io/github/repo-size/sehnryr/oodle-rs)]()

A Rust library for decompressing Kraken, Mermaid, Selkie, Leviathan compressed buffers.

## Development

TODO: explain how to download and extract oodle's source code without using nix flake

Since [oodle-test-data](https://github.com/sehnryr/oodle-test-data) can be quite heavy to clone by itself
(380MB), you can use partial clone and sparse checkout like this:

<details open><summary>in bash:</summary>

```bash
cd test-data

git clone \
    --depth 1 \
    --filter blob:none \
    --sparse https://github.com/sehnryr/oodle-test-data.git .

git sparse-checkout add {raw,kraken,leviathan,mermaid,selkie}
```

</details>

<details><summary>in nushell:</summary>

```nu
cd test-data

(git clone
    --depth 1
    --filter blob:none
    --sparse https://github.com/sehnryr/oodle-test-data.git .)

["raw", "kraken", "leviathan", "mermaid", "selkie"] | each { |name| git sparse-checkout add $"($name)" }
```

</details>
