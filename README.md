# oodle-rs

[![GitHub repo size](https://img.shields.io/github/repo-size/sehnryr/oodle-rs)]()

A Rust library for decompressing Kraken, Mermaid, Selkie, Leviathan compressed buffers.

## Development

Since [OodleUE](https://github.com/WorkingRobot/OodleUE) can be quite heavy to clone by itself
(700MB), you can use partial clone and sparse checkout like this:

<details open><summary>in bash:</summary>

```bash
cd oodle-ue

git clone \
    --depth 1 \
    --filter blob:none \
    --sparse https://github.com/WorkingRobot/OodleUE.git .

git sparse-checkout add Engine/Source/Runtime/OodleDataCompression/Sdks/2.9.13/{help,src}
```

</details>

<details><summary>in nushell:</summary>

```nu
cd oodle-ue

(git clone
    --depth 1
    --filter blob:none
    --sparse https://github.com/WorkingRobot/OodleUE.git .)

["help", "src"] | each { |name| git sparse-checkout add $"Engine/Source/Runtime/OodleDataCompression/Sdks/2.9.13/($name)" }
```

</details>

The same can be applied to [oodle-test-data](https://github.com/sehnryr/oodle-test-data):

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
