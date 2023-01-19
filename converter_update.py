﻿# imports
import glob

test = 1


def read_table():
    table = []
    for line in open(fname, "r").read().strip().splitlines():

        # 1st check if new or old:
        cols = line.split("\t")
        version = "old" if len(cols) == 5 else "new"

        table.append(cols)

    return table, version


def write_table(version):
    # can write a list of lines to a .txt file

    with open(fname, "w+") as f:
        for i, l in enumerate(table):

            line = []

            for i, elem in enumerate(l):
                if i == 0:
                    # waveform
                    line.append(cols[0])
                elif i == 1:
                    # frequency
                    line.append(elem)
                elif i == 2:
                    # Vpp
                    line.append(cols[1])
                elif i == 3:
                    # duration
                    line.append(cols[2])

                line.append("\t")

            if version == "old":
                # 2*, because of the "\t"
                line = line[: 2 * 2] + [str(0) + "\t"] + line[2 * 2 :]

            line = "".join(line).strip()

            f.write(line + "\n") if i < len(table) - 1 else f.write(line)


print("Welcome to the Velleman List Editor!")

selection = "Please select one of the following modi: \n \
            1.) \tFor the conversion of the list in 'frequenzen.txt' press '1'! \n \
            2.) \tFor conversion of old table format (PCGU1000) into the new one (PCSU200) press '2'! \n \
            3.) \tFor conversion of new table format (PCSU200) into the old one (PCGU1000) press '3'! \n \
            4.) \tFor creation of the old table format from the list in 'frequenzen.txt' press '4'! \n \
            5.) \tFor creation of the new table format from the list in 'frequenzen.txt' press '5'!"

print(selection)

modus = input().strip()

try:
    int(modus)
    mod = int(modus)
except:
    print("The key you gave was not a number!")
    input("Press any key to exit!\n")
    exit(0)

# use a partial string from user, to print all similar filenames
partial_str = str(
    input(
        "As a help, type in a fraction of the filename.\n"
        "All similar filenames will be shown:\n"
    )
)

all_sim_files = ""

for s in glob.glob("./*" + partial_str + "*.txt"):
    all_sim_files += s[2:] + "\t"

print(all_sim_files)

# type in the filename to convert, if 1|2|3
if mod < 4:
    fname = str(input("Type in the filename:\n")).strip()

    # check if file-extension is given:
    if ".txt" not in fname:
        fname += ".txt"

    # if test:
    # fname = "Healing.txt"


# to modify the waveform, time, voltage(pp)
if mod > 1:
    dur = input("Type in the duration in [seconds]:\n").strip()
    Vpp = input("\nType in the peak-to-peak voltage in [Volt]:\n").strip()
    waveform = input(
        "\nType in the waveform: [1=sine, 2=rect, 3=tri]\n"
    ).strip()
    cols = [waveform, Vpp, dur]

    for i, s in enumerate(cols):
        if not s.replace(".", "").isdigit():
            input(
                f"The given String {s} is no number - Press any key to exit!\n"
            )
            exit(0)


# -- All modi -- #
if mod == 1:
    # 1.) \tFor the conversion of the list in 'frequenzen.txt'
    # read the table -> save frequencies as list in "frequenzen.txt"
    table, version = read_table()
    # print(version, table)

    # select 2nd column -> frequencies
    freqs = ""
    for line in table:
        freqs += line[1] + ", "

    freqs = f"{fname.replace('.txt', '')}: " + freqs[:-1]

    # save frequencies in "frequenzen.txt"
    with open("frequenzen.txt", "w+") as f:
        f.write(freqs)

if mod == 2:
    # 2.) \tFor conversion of old table format (PCGU1000) into the new one (PCSU200)
    table, version = read_table()

    fname = fname.replace("old", "") + "_new"

    table_c = table[:]
    table = []

    # only eliminate the 3rd column (DC offset=0)
    for line in table_c:
        table.append(line[:2] + line[3:])

    write_table("new")

if mod == 3:
    # 3.) \tFor conversion of new table format (PCSU200) into the old one (PCGU1000)
    table, version = read_table()

    fname = (
        fname.replace(".txt", "").replace("_new", "").replace("_pcsu200", "")
        + "_old.txt"
    )

    write_table("old")

if mod == 4:
    # 4.) \tFor creation of the old table format from the list in 'frequenzen.txt'

    pass

if mod == 5:
    # 5.) \tFor creation of the new table format from the list in 'frequenzen.txt'
    pass