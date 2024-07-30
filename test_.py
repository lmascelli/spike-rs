from pathlib import Path
from pycode.project import Project

if __name__ == '__main__':
    project = Project()
    project.add_batch(Path("/home/leonardo/Documents/unige/data/12-04-2024/38940_DIV77/raw/2024-04-11T14-46-5338940_100E_DIV77_nbasal_0003_E-00155.h5"))
    project.write()
