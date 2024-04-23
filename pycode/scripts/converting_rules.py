from typing import Optional


class ConvertingValues:
    def __init__(self, matrice: str, cond: str, div: str, i: str, t: str):
        self.matrice = matrice
        self.cond = cond
        self.div = div
        self.i = i
        self.t = t

    def __str__(self) -> str:
        return f"""{{
        matrice: {self.matrice},
        cond: {self.cond},
        div: {self.div},
        i: {self.i},
        t: {self.t},
}}"""


def rule1(name: str) -> Optional[ConvertingValues]:
    """
    Example: 01_basal
    {
        matrix: 00000,
        cond: XXX,
        div: 00,
        i: 01,
        t: basal,
    }
    """
    try:
        i = name[: name.find("_")]
        t = name[name.find("_") + 1 : -3]
        return ConvertingValues("00000", "XXX", "00", i, t)
    except:
        return None


def rule2(name: str) -> Optional[ConvertingValues]:
    """
    Example: 2024-04-11T14-31-1938940_100E_DIV77_nbasal_0001_E-00155.h5
    {
        matrix: 38940,
        cond: 100E,
        div: 77,
        i: 01,
        t: nbasal,
    }
    """
    try:
        first_ = name.find("_") + 1
        matrice = name[first_ - 6 : first_ - 1]
        second_ = name.find("_", first_) + 1
        cond = name[first_ : second_ - 1]
        third_ = name.find("_", second_) + 1
        div = name[name.find("DIV") + 3 : third_ - 1]
        fourth_ = name.find("_", third_) + 1
        t = name[third_ : fourth_ - 1]
        fifth_ = name.find("_", fourth_) + 1
        i = str(int(name[fourth_ : fifth_ - 1]))
        return ConvertingValues(matrice, cond, div, f"000{i}", t)
    except:
        return None
