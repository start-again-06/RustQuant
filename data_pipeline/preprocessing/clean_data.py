import pandas as pd

def clean_data(df):

    df = df.dropna()

    df = df.reset_index(drop=True)

    return df
