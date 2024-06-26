[![Build Badge]][build] [![Crates Badge]][crates] [![Docs Badge]][docs] [![License Badge]][license]

[Build Badge]: https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Ffrank-leap%2Fmatchina%2Fbadge%3Fref%3Dmain&style=flat&label=build
[build]: https://actions-badge.atrox.dev/frank-leap/matchina/goto?ref=main

[Crates Badge]: https://img.shields.io/crates/v/matchina.svg
[crates]: https://crates.io/crates/matchina

[Docs Badge]: https://docs.rs/matchina/badge.svg
[docs]: https://docs.rs/matchina

[License Badge]: https://img.shields.io/badge/License-MIT-blue.svg
[license]: LICENSE

# matchina

This is a minimalistic matching engine designed for a trading exchange. It supports various order types including limit orders, market orders, and order features such as Immediate-Or-Cancel (IOC), Fill-Or-Kill (FOK), and Post-Only orders.

## Features

- **Limit Orders:** Traders can place buy or sell orders at specified price levels, ensuring their orders are executed at their desired prices or better.
- **Market Orders:** Traders can place orders to be executed at the current market price, guaranteeing an immediate fill.
- **Immediate-Or-Cancel (IOC):** IOC orders are designed for immediate execution. Any portion of an IOC order that cannot be filled immediately is canceled.
- **Fill-Or-Kill (FOK):** FOK orders demand complete execution. If the entire order cannot be filled immediately, it is canceled.
- **Post-Only Orders:** Post-Only orders are added to the order book and are only executed as maker orders, ensuring no additional fees as a taker.

## Usage

To run the simulation with this matching engine, simply execute the following command:

```shell
just run
```

[Just](https://github.com/casey/just) is used to manage various build and development tasks, and you can explore the available options with:

```shell
just
```

## Contributing

Contributions from the community are welcomed!
Feel free to submit bug reports, feature requests, or even pull requests to enhance the matching engine.

## License

This project is licensed under the [MIT License](LICENSE).
