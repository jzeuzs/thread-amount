# thread-amount

[![Crate Version](https://img.shields.io/crates/v/thread-amount)](https://crates.io/crates/thread-amount)
[![Documentation](https://docs.rs/thread-amount/badge.svg)](https://docs.rs/thread-amount)
[![License](https://img.shields.io/crates/l/thread-amount.svg)](./LICENSE-APACHE)
[![Continuous Delivery](https://github.com/jzeuzs/thread-amount/actions/workflows/continuous-delivery.yml/badge.svg)](https://github.com/jzeuzs/thread-amount/actions/workflows/continuous-delivery.yml)
[![Continuous Integration](https://github.com/jzeuzs/thread-amount/actions/workflows/continuous-integration.yml/badge.svg)](https://github.com/jzeuzs/thread-amount/actions/workflows/continuous-integration.yml)

**Get the amount of threads in the current process**

## Example Usage

`Cargo.toml`

```toml
[dependencies]
thread-amount = "0.1"
```

The code:

```rust
use thread_amount::thread_amount;

use std::thread;

fn main() {
    let amount = thread_amount();

    thread::spawn(move || {
        assert_eq!(amount.map(NonZeroUsize::get), Some(1))
    });
}
```

## Difference between [`num-threads`](https://crates.io/crates/num-threads)

This crate has windows support and reads the `/proc/[PID]/status` file in `-unix` systems.

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):
<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/jzeuzs"><img src="https://avatars.githubusercontent.com/u/75403863?v=4?s=100" width="100px;" alt="jez"/><br /><sub><b>jez</b></sub></a><br /><a href="https://github.com/jzeuzs/thread-amount/commits?author=jzeuzs" title="Code">💻</a> <a href="https://github.com/jzeuzs/thread-amount/commits?author=jzeuzs" title="Documentation">📖</a> <a href="#ideas-jzeuzs" title="Ideas, Planning, & Feedback">🤔</a> <a href="#maintenance-jzeuzs" title="Maintenance">🚧</a> <a href="#platform-jzeuzs" title="Packaging/porting to new platform">📦</a> <a href="https://github.com/jzeuzs/thread-amount/commits?author=jzeuzs" title="Tests">⚠️</a></td>
    </tr>
  </tbody>
  <tfoot>
    <tr>
      <td align="center" size="13px" colspan="7">
        <img src="https://raw.githubusercontent.com/all-contributors/all-contributors-cli/1b8533af435da9854653492b1327a23a4dbd0a10/assets/logo-small.svg">
          <a href="https://all-contributors.js.org/docs/en/bot/usage">Add your contributions</a>
        </img>
      </td>
    </tr>
  </tfoot>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!

## License

Licensed under the MIT license ([LICENSE](LICENSE) or <http://opensource.org/licenses/MIT>)
