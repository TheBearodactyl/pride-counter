# how to run:

windows:
```powershell
./run.ps1
```

linux/mac:
```bash
./run.sh
```

# how to add countries:

in `src/main.rs`, add a new entry to the `pride_countries` vector, with the following fields:

```rust
get_country_stats("Country Name", CURRENT_YEAR - /* Year this country started hosting Pride Parades */),
```

Make sure that the `Country Name` is the same as the name of the country on Wikipedia, and that the `Year` is the year the country started hosting Pride Parades.

How to get a countries name:

1. Search up "List of cities in [country name]" on Wikipedia.
2. Go to the link for the country's list of cities.
3. At the end of the URL, you'll see a string like `List_of_cities_in_[country name]`. Copy the `[country name]` part.
4. This will be the `Country Name` in the `get_country_stats` function.

How to get the year of the country's first Pride Parade:

1. google it.
2. Copypasta.
