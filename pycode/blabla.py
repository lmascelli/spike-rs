import pycode as pc
from mc_explorer import MCExplorer
from os import mkdir, listdir

from pathlib import Path

# BASAL = 0
# DIGITAL = 1
# STIM = 2
# 
base_folder = Path("/home/leonardo/Documents/unige/raw data/12-04-2024/")
# matrix_name = "38936"
# div = "77"
# i = "04"
# # t = "Stim_50"
# # t = "basal"
# t = "StimEl"
# batch_folder= f"{matrix_name}_DIV{div}"  
# filename = f"{i}_{t}.h5"
# 
# dest_ = f"/home/leonardo/Documents/unige/raw data/12-04-2024/38936_DIV77/"
# dest = f"/home/leonardo/Documents/unige/raw data/12-04-2024/38936_DIV77/mat_{i}_{t}"
# mkdir(f"{dest_}/mat_{i}_{t}")
# explorer = MCExplorer(base_folder.joinpath(batch_folder).joinpath(filename))
# print(explorer)
# # convert = DIGITAL
# if t == "basal":
#     phase: pc.PyPhase = explorer.convert_phase(0, 0, None, None)
#     phase.save(base_folder.joinpath(batch_folder).joinpath('hdf5').joinpath(filename))
#     phase.save_as_mat(matrix_name, '100E', div, i, t, dest)
# if t == "StimEl":
#     phase = explorer.convert_phase(0, 0, None, 0)
#     phase.save(base_folder.joinpath(batch_folder).joinpath('hdf5').joinpath(filename))
#     phase.save_as_mat(matrix_name, '100E', div, i, t, dest)
# if t.startswith("Stim_"):
#     phase = explorer.convert_phase(0, 0, 1, None)
#     phase.save(base_folder.joinpath(batch_folder).joinpath('hdf5').joinpath(filename))
#     phase.save_as_mat(matrix_name, '100E', div, i, t, dest)

for folder in listdir(base_folder):
    current_folder = base_folder.joinpath(folder)
    matrix_name = current_folder.name[:current_folder.name.find('_')]
    div = current_folder.name[current_folder.name.find('DIV')+3:]
    try:
        mkdir(current_folder.joinpath('hdf5'))
    except:
        pass

    for file in listdir(current_folder):
        if file.endswith('.h5'):
            _i = file.find('_', file.find('77_') + 3)
            i_start = file.find('_', _i) + 1
            i_end = file.find('_', i_start)
            i = "{:02d}".format(int(file[i_start:i_end]))
            t = file[file.find('DIV')+6:file.find('_00')]
            file = current_folder.joinpath(file)
            savefile = current_folder.joinpath('hdf5').joinpath(f'{matrix_name}_100E_DIV{div}_{t}_00{i}.h5')
            mat_folder = current_folder.joinpath(f'mat_{t}_{i}')
            mkdir(mat_folder)
            print(mat_folder)
            phase = pc.PyPhase.from_file(savefile)
            phase.save_as_mat(matrix_name, '100E', div, i, t, mat_folder)
            # explorer = MCExplorer(file.absolute())
            if t == 'nbasal':
                pass
                # phase: pc.PyPhase = explorer.convert_phase(0, 0, None, None)
                # del explorer
                # print('BASAL', file)
                # phase.save(savefile)
            elif t.upper().find('STIMEL') > -1:
                pass
                # phase: pc.PyPhase = explorer.convert_phase(0, 0, None, 0)
                # del explorer
                # print('STIMEL', file)
                # phase.save(savefile)
            else:
                pass
                # phase: pc.PyPhase = explorer.convert_phase(0, 0, 1, None)
                # del explorer
                # print('DIGITAL', file)
                # phase.save(savefile)
