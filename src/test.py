def t():
    with open("test.txt", "r") as f:
        with open("r.txt", "w") as g:
            for line in f.readlines():
                if line.strip().startswith("/"):
                    continue

                a = "".join(x.strip().capitalize() for x in line.split("_"))
                g.write(a)
                g.write('\n')

