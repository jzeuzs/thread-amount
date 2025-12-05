# Changelog

## [0.3.0](https://github.com/jzeuzs/thread-amount/compare/v0.2.2...v0.3.0) (2025-12-05)


### Features

* switch to Rust edition 2024 ([#32](https://github.com/jzeuzs/thread-amount/issues/32)) ([739b167](https://github.com/jzeuzs/thread-amount/commit/739b1673342c09175043c30ab757a840a4cd16a5))

## [0.2.2](https://github.com/jzeuzs/thread-amount/compare/v0.2.1...v0.2.2) (2025-11-21)


### Bug Fixes

* memory leak on windows, macos, and ios ([#29](https://github.com/jzeuzs/thread-amount/issues/29)) ([28860d4](https://github.com/jzeuzs/thread-amount/commit/28860d4a38286609cb884c13b5b7941edc2390e5))

## [0.2.1](https://github.com/jzeuzs/thread-amount/compare/v0.2.0...v0.2.1) (2025-11-16)


### Bug Fixes

* **deps:** update rust crate mach2 to 0.6.0 ([#25](https://github.com/jzeuzs/thread-amount/issues/25)) ([f94aebf](https://github.com/jzeuzs/thread-amount/commit/f94aebf9649a3ccc7541b65961e91e61b21f3178))

## [0.2.0](https://github.com/jzeuzs/thread-amount/compare/v0.1.3...v0.2.0) (2025-11-07)


### Features

* add support for osx ([379234d](https://github.com/jzeuzs/thread-amount/commit/379234d4ea9be3bfc30ce4b97d68835aa9c7e970))


### Bug Fixes

* `clippy::pedantic` warnings ([03b32bf](https://github.com/jzeuzs/thread-amount/commit/03b32bf8db59ea2f35360882e0ce0e58b533362c))
* **deps:** update all non-major dependencies ([#13](https://github.com/jzeuzs/thread-amount/issues/13)) ([ac7ef0b](https://github.com/jzeuzs/thread-amount/commit/ac7ef0b2eb28916e5e3575f33f56bb438e174c86))
* **deps:** update all non-major dependencies ([#21](https://github.com/jzeuzs/thread-amount/issues/21)) ([7c58a6b](https://github.com/jzeuzs/thread-amount/commit/7c58a6b2a03baa6400e7cf929c3a293939ad920a))
* update windows features, osx pointer, unix `cfg` ([#19](https://github.com/jzeuzs/thread-amount/issues/19)) ([3579f5c](https://github.com/jzeuzs/thread-amount/commit/3579f5cd94cc41a9f757c334cb5352e72cd249a8))

## [0.1.3](https://github.com/devtomio/thread-amount/compare/v0.1.2...v0.1.3) (2022-06-24)


### Bug Fixes

* **deps:** update rust crate windows to 0.38.0 ([#9](https://github.com/devtomio/thread-amount/issues/9)) ([e1143ac](https://github.com/devtomio/thread-amount/commit/e1143ac2aa7d40de1aea950b4bc84fc8977d80b2))

### [0.1.2](https://github.com/devtomio/thread-amount/compare/v0.1.1...v0.1.2) (2022-06-01)


### Bug Fixes

* **docs:** clarify the difference between this crate and `num-threads` ([07fffa2](https://github.com/devtomio/thread-amount/commit/07fffa2c5dad980b2b2c7ff6179ca37af7f7cb65))

### [0.1.1](https://github.com/devtomio/thread-amount/compare/v0.1.0...v0.1.1) (2022-06-01)


### Bug Fixes

* non-existent file in readme ([a35ce29](https://github.com/devtomio/thread-amount/commit/a35ce2911a6246e59396d77c9577499e0c866dd2))

## 0.1.0 (2022-06-01)


### Features

* init library ([40d328b](https://github.com/devtomio/thread-amount/commit/40d328b75438a3247bf41171089a5153760532f6))
* use `procfs` for unix ([2941533](https://github.com/devtomio/thread-amount/commit/294153329a7cfa7dc196e784d85c01dd72a3b4f5))


### Bug Fixes

* change switched up `cfg_attr` macro ([d8261dd](https://github.com/devtomio/thread-amount/commit/d8261dd53cf250662aada1f1d0c03425d042bb5c))
* read `proc/[PID]/status` manually ([6dd7ba3](https://github.com/devtomio/thread-amount/commit/6dd7ba38336624895860e17a21cbef3d3ef9148c))
