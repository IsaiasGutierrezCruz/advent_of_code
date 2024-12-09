import polars as pl

def get_sum_of_calibration_values(df: pl.DataFrame) -> int:
    return df.select(
        pl.col("input_text")
        .str.extract_all(r"(\d)")
    ).with_columns(
        pl.col("input_text").list.first() + 
        pl.col("input_text").list.last()
    ).cast(pl.Int64).sum().get_column("input_text")[0]



if __name__ == "__main__":
    df_input = pl.read_csv("input.csv")
    result = get_sum_of_calibration_values(df=df_input)
    print(result)