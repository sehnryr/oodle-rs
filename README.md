# oodle-rs

[![GitHub repo size](https://img.shields.io/github/repo-size/sehnryr/oodle-rs)]()

A Rust library for decompressing Kraken, Mermaid, Selkie, Leviathan compressed buffers.

## Development

Since [OodleUE](https://github.com/WorkingRobot/OodleUE) can be quite heavy to clone by itself
(700MB), you can use partial clone and sparse checkout like this:

```bash
cd oodle-ue

git clone \
    --depth 1 \
    --filter blob:none \
    --sparse https://github.com/WorkingRobot/OodleUE.git .

git sparse-checkout add Engine/Source/Runtime/OodleDataCompression/Sdks/2.9.13/{help,src}
```

The same can be applied to [oodle-test-data](https://github.com/sehnryr/oodle-test-data):

```bash
cd test-data

git clone \
    --depth 1 \
    --filter blob:none \
    --sparse https://github.com/sehnryr/oodle-test-data.git .

git sparse-checkout add {raw,kraken,leviathan,mermaid,selkie}
```
