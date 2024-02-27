YARER GUI - A gui for Yarer
===========================

[<img alt="github" src="https://img.shields.io/badge/github-davassi/davassi?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/davassi/yarer-gui)
[<img alt="build status" src="https://github.com/davassi/yarer-gui/actions/workflows/rust.yml/badge.svg" height="20">](https://github.com/davassi/yarer-gui/actions?query=branch%3Amaster)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/yarer?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/yarer-gui)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)

Yarer Gui is a frontend for Yarer, a library I've written (https://github.com/davassi/yarer) for the evaluation of mathematical expressions, using Reverse Polish Notation.

Screenshots:

![image](https://github.com/davassi/yarer-gui/assets/1568018/b4b85dbc-31e0-4581-9e58-6b6a1dff70c2)

## Notes

When you start the app, if you notice the error: "XDG Settings Portal did not return response in time: timeout: 100ms, key: color-scheme" or similar,

please try to start the app with setting up the following variable:

```console
$ WINIT_UNIX_BACKEND="x11" cargo run -q -- 
```

## Contribution

Besides being stable, Yarer-GUI is a work in progress. If you have suggestions for features (i.e. more math functions to implement), or if you find any issues in the code, design, interface, etc, please feel free to share them on our [GitHub](https://github.com/davassi/yarer-gui/issues).

I appreciate very much your feedback!

ca
