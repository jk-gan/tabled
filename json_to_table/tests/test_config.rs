use json_to_table::json_to_table;
use serde_json::json;
use tabled::{Alignment, Padding, Style, Table};

#[cfg(feature = "color")]
use tabled::papergrid::{AnsiColor, GridConfig};

#[test]
fn config_from_table_test() {
    let value = json!(
        {
            "key1": 123,
            "234": ["123", "234", "456"],
            "key22": {
                "k1": 1,
                "k2": 2,
            }
        }
    );

    let cfg = Table::new([""])
        .with(Padding::zero())
        .with(Alignment::center())
        .with(Alignment::center_vertical())
        .get_config()
        .clone();

    let table = json_to_table(&value)
        .set_style(Style::modern())
        .set_config(cfg)
        .collapse()
        .to_string();

    println!("{}", table);

    assert_eq!(
        table,
        concat!(
            "┌───────┬──────┐\n",
            "│       │ 123  │\n",
            "│       ├──────┤\n",
            "│  234  │ 234  │\n",
            "│       ├──────┤\n",
            "│       │ 456  │\n",
            "├───────┼──────┤\n",
            "│ key1  │ 123  │\n",
            "├───────┼────┬─┤\n",
            "│       │ k1 │1│\n",
            "│ key22 ├────┼─┤\n",
            "│       │ k2 │2│\n",
            "└───────┴────┴─┘",
        )
    );
}

#[test]
fn config_from_table_general_test() {
    let value = json!(
        {
            "key1": 123,
            "234": ["123", "234", "456"],
            "key22": {
                "k1": 1,
                "k2": 2,
            }
        }
    );

    let cfg = Table::new([""])
        .with(Padding::zero())
        .with(Alignment::center())
        .with(Alignment::center_vertical())
        .get_config()
        .clone();

    let table = json_to_table(&value)
        .set_style(Style::modern())
        .set_config(cfg)
        .to_string();

    println!("{}", table);

    assert_eq!(
        table,
        concat!(
            "┌─────┬──────┐\n",
            "│     │┌───┐ │\n",
            "│     ││123│ │\n",
            "│     │├───┤ │\n",
            "│ 234 ││234│ │\n",
            "│     │├───┤ │\n",
            "│     ││456│ │\n",
            "│     │└───┘ │\n",
            "├─────┼──────┤\n",
            "│key1 │ 123  │\n",
            "├─────┼──────┤\n",
            "│     │┌──┬─┐│\n",
            "│     ││k1│1││\n",
            "│key22│├──┼─┤│\n",
            "│     ││k2│2││\n",
            "│     │└──┴─┘│\n",
            "└─────┴──────┘",
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn color_test() {
    let value = json!(
        {
            "key1": 123,
            "234": ["123", "234", "456"],
            "key22": {
                "k1": 1,
                "k2": 2,
            }
        }
    );

    let mut cfg = GridConfig::default();
    cfg.set_border_color_global(AnsiColor::new(
        String::from("\u{1b}[34m"),
        String::from("\u{1b}[39m"),
    ));

    let table = json_to_table(&value)
        .set_style(Style::modern())
        .set_config(cfg)
        .collapse()
        .to_string();

    println!("{}", table);

    assert_eq!(
        table,
        concat!(
            "\u{1b}[34m┌───────\u{1b}[39m\u{1b}[34m┬──────┐\u{1b}[39m\n",
            "\u{1b}[34m│\u{1b}[39m234    \u{1b}[34m│\u{1b}[39m123   \u{1b}[34m│\u{1b}[39m\n",
            "\u{1b}[34m│\u{1b}[39m       \u{1b}[34m├──────┤\u{1b}[39m\n",
            "\u{1b}[34m│\u{1b}[39m       \u{1b}[34m│\u{1b}[39m234   \u{1b}[34m│\u{1b}[39m\n",
            "\u{1b}[34m│\u{1b}[39m       \u{1b}[34m├──────┤\u{1b}[39m\n",
            "\u{1b}[34m│\u{1b}[39m       \u{1b}[34m│\u{1b}[39m456   \u{1b}[34m│\u{1b}[39m\n",
            "\u{1b}[34m├───────\u{1b}[39m\u{1b}[34m┼──────┤\u{1b}[39m\n",
            "\u{1b}[34m│\u{1b}[39mkey1   \u{1b}[34m│\u{1b}[39m123   \u{1b}[34m│\u{1b}[39m\n",
            "\u{1b}[34m├───────\u{1b}[39m\u{1b}[34m┼────┬─┤\u{1b}[39m\n",
            "\u{1b}[34m│\u{1b}[39mkey22  \u{1b}[34m│\u{1b}[39mk1  \u{1b}[34m│\u{1b}[39m1\u{1b}[34m│\u{1b}[39m\n",
            "\u{1b}[34m│\u{1b}[39m       \u{1b}[34m├────\u{1b}[39m\u{1b}[34m┼─┤\u{1b}[39m\n",
            "\u{1b}[34m│\u{1b}[39m       \u{1b}[34m│\u{1b}[39mk2  \u{1b}[34m│\u{1b}[39m2\u{1b}[34m│\u{1b}[39m\n",
            "\u{1b}[34m└───────\u{1b}[39m\u{1b}[34m┴────\u{1b}[39m\u{1b}[34m┴─┘\u{1b}[39m",
        )
    );
}
