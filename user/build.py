from os import listdir, system

base_address = 0x80400000
step = 0x20000
linker = 'src/linker.ld'
linker_bak = 'src/linker.ld.bak'
apps = listdir("src/bin")
apps.sort()

for app_id, app in enumerate(apps):
    name = app[:app.find('.')]
    with open(linker, 'w') as linker_fp:
        with open(linker_bak, 'r') as prototype_fp:
            for line in prototype_fp.readlines():
                linker_fp.write(line.replace(hex(base_address), hex(base_address+step*app_id)))
    system('cargo build --bin %s --release' % name)
    system('rm '+linker)
    print('[build.py] application %s start with address %s' %(name, hex(base_address+step*app_id)))
    

