# Crossgate Graphic

The graphic parser for crossgate online game.

## Usage

- Requirement
    - GraphicInfo.bin
    - Graphic.bin
    - Palet.cgp

Files are usually under `./bin/` and `./bin/pal`.

```bash
cargo build
cargo run [GraphicInfo.bin] [Graphic.bin] [Palet.cgp]
```

**注意事項**: 
- 本解析器目前僅針對「神獸傳奇 + 魔弓傳奇」的版本開發（適用於 `GraphicInfo_xx.bin` 及 `Graphic_xx.bin` ）
- 理論上將可以解析「龍之沙漏」版本 `GraphicInfoEx_xx.bin`, `GraphicEx_xx.bin`
- 因檔案結構不同，故目前完全不支援「樂園之卵」及以後所有版本
- 目前功能仍在開發中，尚不完備敬請見諒

## References

1. [御劍軒 - 魔力寶貝檔案結構分析 ── 圖像、地圖](https://cgsword.com/filesystem_graphicmap.htm)
2. [魔力宝贝高清单机计划（一） 图库提取](https://blog.csdn.net/qq_37543025/article/details/88377553)

## LICENSE

This project is under [GPLv2](https://www.gnu.org/licenses/old-licenses/gpl-2.0.html) license.