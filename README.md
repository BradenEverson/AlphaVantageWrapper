# AlphaVantageWrapper

Stock analysis API wrapper crate for rust! Get real time data on publicly accessible stock information

## Installation

Use the package manager [cargo](https://crates.io/) to add AlphaVantageWrapper to your rust project.

```bash
cargo add AlphaVantageWrapper
```

or add the dependency directly in your **cargo.toml** file

```toml
[dependencies]
AlphaVantageWrapper = "{version}"
```

## Usage

```rust
use reqwest::Error;
use AlphaVantageWrapper::broker::broker_api::BrokerAPI;

#[tokio::main]
async fn main() -> Result<(),Error>{
    let alpha_api = BrokerAPI::new("[API KEY]");

    Ok(())
}
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
