# imports
import numpy as np
import ctypes
import os
import re

#
def unique(a):
    unique_a = []
    for i in a:
        if i not in unique_a:
            unique_a.append(i)
    return unique_a


#
def seperate_table(s, flag):
    # print(s)
    # exit()
    a = s.rsplit("\t")
    # print(a)
    #  b = a.split('\t')
    # print('Die seperierte Liste:')
    # print(a)
    # exit()
    if flag == "old":
        freq = np.zeros((round(len(a) / 4)))
        for i in range(0, round(len(a) / 4) - 1):
            freq[i] = float(a[4 * i + 1])
    elif flag == "new":
        freq = np.zeros((round(len(a) / 4)))
        for i in range(0, round(len(a) / 4) - 1):
            freq[i] = float(a[3 * i + 1])
    return freq


#
def write_list(filename, a):
    s = ""
    filename = filename  # + ".txt"
    # print(a)
    for i in range(0, len(a) - 1):
        s = s + str(a[i]) + ","
    # Der filename wird um 4 Stellen gekürzt: '.txt'
    s = filename[0:-4] + ":," + s[0:-1]
    # print(s)
    f = open("frequenzen.txt", "w+")
    f.write(s)
    f.close()
    # exit()


#
def seperate_list(s):
    # print(s.find(','))
    if s.find(";") != -1:
        a = s.rsplit(";")
    elif s.find(",") != -1:
        a = s.rsplit(",")
    # print('Die seperierte Liste:')
    # print(a)
    # exit()
    name = str(a[0])
    name = str(name[:-1])
    # look if 'pcsu_200' or 'pcsu_2000' is already included
    pos_new = name.find("200")
    pos_old = name.find("2000")
    # print(name, pos_new, pos_old)
    # exit()
    if pos_new == -1:
        flag = "old"
        if pos_old != -1:
            name = name[:pos_old]
        # else:
        # name = name[:-1]
    elif pos_new != -1:
        flag = "new"
        name = name[:pos_new]
    a = a[1:]
    # exit()
    return a, name, flag


#
def create(freq, name, flag):
    t = int(input("\nGeben Sie bitte die Dauer pro Frequenz ein!\n"))
    v = float(input("\nGeben Sie bitte die peak-to-peak Spannung ein!\n"))
    # Hier kann man die Parameter ändern!
    if flag == "new":
        liste = np.zeros((len(freq), 4))
        for i in range(0, len(freq) - 1):
            liste[i][0] = int(1)
            liste[i][1] = float(freq[i])
            liste[i][2] = float(v)
            liste[i][3] = int(t)
    elif flag == "old":
        liste = np.zeros((len(freq), 5))
        for i in range(0, len(freq) - 1):
            liste[i][0] = int(1)
            liste[i][1] = float(freq[i])
            liste[i][2] = 0
            liste[i][3] = float(v)
            liste[i][4] = int(t)
    write_table(name, liste, flag)


#
def write_table(filename, a, flag):
    num = ""
    # print(filename)
    if flag == "old":
        filename = filename + "_" + flag + ".txt"
    elif flag == "new":
        filename = filename + "_" + flag + ".txt"
    # print(a)
    for i in range(0, len(a) - 1):
        for j in range(0, len(a[i]) - 1):
            num = num + str(a[i][j]) + "\t"
        num = num + str(a[i][j + 1]) + "\n"
    f = open(filename, "w+")
    f.write(num)
    f.close()


#
## Tabelle in Liste konvertieren
selection = 'Dies ist das Auswahlmenü: \n \
            1.) \tZum Konvertieren der Tabelle in die Datei "frequenzen.txt" geben Sie bitte die "1" ein! \n \
            2.) \tWollen Sie die alte (PCGU1000) Tabelle in die neue (PCSU200) Tabellenstruktur konvertieren, geben Sie bitte die "2" ein! \n \
            3.) \tWollen Sie die neue (PCSU200) Tabelle in die alte (PCGU1000) Tabellenstruktur konvertieren, geben Sie bitte die "3" ein! \n \
            4.) \tWollen Sie aus der Datei "frequenzen.txt" eine Tabelle für den "PCGU1000 (das alte Gerät)" erstellen, geben Sie bitte die "4" ein! \n \
            5.) \tWollen Sie aus der Datei "frequenzen.txt" eine Tabelle für den "PCSU200 (das neue Gerät)" erstellen, geben Sie bitte die "5" ein! \n \
            \n\n\nDies sind alle Dateien im Verzeichnis:'
"""
# fullscreen
kernel32 = ctypes.WinDLL('kernel32')
user32 = ctypes.WinDLL('user32')
print(kernel32)
SW_MAXIMIZE = 3

hWnd = kernel32.GetConsoleWindow()
user32.ShowWindow(hWnd, SW_MAXIMIZE)
"""
# save path and all filenames in variable
all_filenames = os.listdir(path=".")
print(selection)
print(all_filenames, "\n")
key = str(input())
#
if key == "1":
    name = str(
        input(
            "\n Bitte geben Sie den Namen der zu konvertierenden Datei an! \n"
        )
    )
    if re.search(r"(.txt)", name) != None:
        table = name
        e = re.search(r"(.txt)", name).span()
        name = name[: e[0]]
    else:
        table = name + ".txt"
    li = open(table, "r+")
    s = li.read()
    li.close()
    # print(s)
    # build 2 cases! old and new!
    flag = "old"
    freq_list = seperate_table(s, flag)
    name_new = name + ".txt"
    # print('Frequenzreihe:')
    # print(freq_list)
    write_list(name_new, freq_list)
    print('Die Liste "frequenzen.txt" wurde erstellt!')
#
elif key == "2":
    name = str(
        input(
            "\n Bitte geben Sie den Namen der zu konvertierenden Datei an! \n"
        )
    )
    # print(name, 'What is matched? ', re.match(r'([A-Za-z_]*)(.txt)', name).span(1))
    if re.search(r"(.txt)", name) != None:
        table = name
        e = re.search(r"(.txt)", name).span()
        name = name[: e[0]]
    else:
        table = name + ".txt"
    li = open(table, "r+")
    s = li.read()
    li.close()
    # print(s)
    s_np = np.array(re.split(r"\n", s))
    # print(s_np)
    # print(re.findall(r'\t', s_np[0]))
    if len(re.findall(r"\t", s_np[0])) == 4:
        flag = "old"
    if len(re.findall(r"\t", s_np[0])) == 3:
        flag = "new"
        exit("\nIt is already the NEW structure!\n")
    # flag = 'old'
    freq_list = seperate_table(s, flag)
    name_new = name + ".txt"
    # print('Frequenzreihe:')
    # print(freq_list)
    write_list(name_new, freq_list)
    ## Die Liste in die neue Tabellenstruktur konvertieren:
    list_freq = "frequenzen" + ".txt"
    li = open(list_freq, "r+")
    s = li.read()
    li.close()
    # print(s)
    list_arr = seperate_list(s)
    # freq_only = list_arr[0]
    # for i in 1..len(list_arr)-1:
    #  name = list_arr.delete_at(i)
    # list_arr[2]
    name = list_arr[1]
    just_num = list_arr[0]
    # print('Liste ohne Namen')
    # print(list_arr[1], name)
    # print(just_num)
    # exit()
    arr_int = []
    for i in just_num:
        arr_int.append(float(i))
    # print(arr_int)
    # exit()
    final_list = unique(arr_int)
    # print('Finale Liste:')
    # print(final_list)
    # exit()
    flag = "new"
    create(final_list, name, flag)
    print("Die konvertierte Tabelle wurde erstellt!")
#
elif key == "3":
    name = str(
        input(
            "\n Bitte geben Sie den Namen der zu konvertierenden Datei an! \n"
        )
    )
    if re.search(r"(.txt)", name) != None:
        table = name
        e = re.search(r"(.txt)", name).span()
        name = name[: e[0]]
    else:
        table = name + ".txt"
    li = open(table, "r+")
    s = li.read()
    li.close()
    # print(s)
    s_np = np.array(re.split(r"\n", s))
    # print(s_np)
    # print(re.findall(r'\t', s_np[0]))
    if len(re.findall(r"\t", s_np[0])) == 4:
        flag = "old"
        exit("\nIt is already the OLD structure!\n")
    if len(re.findall(r"\t", s_np[0])) == 3:
        flag = "new"
    # flag = 'new'
    freq_list = seperate_table(s, flag)
    name_new = name + ".txt"
    # print('Frequenzreihe:')
    # print(freq_list)
    write_list(name_new, freq_list)
    ## Die Liste in die neue Tabellenstruktur konvertieren:
    list_freq = "frequenzen" + ".txt"
    li = open(list_freq, "r+")
    s = li.read()
    li.close()
    # print(s)
    list_arr = seperate_list(s)
    # freq_only = list_arr[0]
    # for i in 1..len(list_arr)-1:
    #  name = list_arr.delete_at(i)
    # list_arr[2]
    name = list_arr[1]
    just_num = list_arr[0]
    # print('Liste ohne Namen')
    # print(list_arr[1], name)
    # print(just_num)
    # exit()
    arr_int = []
    for i in just_num:
        arr_int.append(float(i))
    # print(arr_int)
    # exit()
    final_list = unique(arr_int)
    # print('Finale Liste:')
    # print(final_list)
    # exit()
    flag = "old"
    create(final_list, name, flag)
    print("Die konvertierte Tabelle wurde erstellt!")
#
elif key == "4":
    ## Die Liste in die alte Tabellenstruktur konvertieren:
    list_freq = "frequenzen" + ".txt"
    li = open(list_freq, "r+")
    s = li.read()
    li.close()
    # print(s)
    list_arr = seperate_list(s)
    # freq_only = list_arr[0]
    # for i in 1..len(list_arr)-1:
    #  name = list_arr.delete_at(i)
    # flag = list_arr[2]
    flag = "old"
    name = list_arr[1]
    just_num = list_arr[0]
    # print('Liste ohne Namen')
    # print(list_arr[1], name)
    # print(just_num)
    # exit()
    arr_int = []
    for i in just_num:
        arr_int.append(float(i))
    # print(arr_int)
    # exit()
    final_list = unique(arr_int)
    # print('Finale Liste:')
    # print(final_list)
    # exit()
    create(final_list, name, flag)
    print("Die konvertierte Tabelle wurde erstellt!")
#
elif key == "5":
    ## Die Liste in die neue Tabellenstruktur konvertieren:
    list_freq = "frequenzen" + ".txt"
    li = open(list_freq, "r+")
    s = li.read()
    li.close()
    list_arr = seperate_list(s)
    # flag = list_arr[2]
    flag = "new"
    name = list_arr[1]
    just_num = list_arr[0]
    # print('Liste ohne Namen')
    # print(list_arr[1], name)
    # print(just_num)
    # exit()
    arr_int = []
    for i in just_num:
        arr_int.append(float(i))
    # print(arr_int)
    # exit()
    final_list = unique(arr_int)
    # print('Finale Liste:')
    # print(final_list)
    # exit()
    create(final_list, name, flag)
    print("Die konvertierte Tabelle wurde erstellt!")
input("Um zu Beenden, drücken sie irgendeine Taste auf der Tastatur!")
