# LD52

## Veggies

| Veggie                            | Constraints                                                                                                   |
|-----------------------------------|---------------------------------------------------------------------------------------------------------------|
| ![Strawberry](promo/veggies1.png) | - **invasive:** Loves being next to other strawberries                                                        |
| ![Carrot](promo/veggies4.png)     | - **pure:** Hates being next to a rock                                                                        |
| ![Garlic](promo/veggies6.png)     | - **dry:** Hates being next to water                                                                          |
| ![Apple](promo/veggies3.png)      | - **foliage:** Its leaves bother adjacent veggies                                                             |
| ![Mint](promo/veggies7.png)       | - **tangled:** Its roots bother adjacent carrots and garlic                                                   |
| ![Tomato](promo/veggies2.png)     | - **protected:** Loves being next to garlic or carrots                                                        |
| ![Cherry](promo/veggies5.png)     | - **pairs:** Loves being next to exactly one cherry<br/>- **alone:** Hates apple trees in its line and column |

## Build, Run

### Run locally

```bash
cargo run
```

### Build for the web

```bash
trunk build --release
```

At this point the build can be tested with:

```bash
basic-http-server dist
```

**Important:** To make the `dist/` folder compatible with itch.io, make sure all links in `dist/index.html` are prefixed with `./`.