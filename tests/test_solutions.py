from adventcode.day1.day1 import get_sum_of_calibration_values
import polars as pl
from pathlib import Path

here = Path(__file__).resolve().parent

def test_day1_get_sum_of_calibration_values() -> None: 
    df_input = pl.read_csv(here / "test_data/input_day1.csv")
    expected_result = 142

    result = get_sum_of_calibration_values(df=df_input)
    assert result == expected_result

