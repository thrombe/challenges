
class Ite:
    def __init__(self, s):
        self.s = s
        self.be = []
        self.fe = []
    def next(self):
        if len(self.s) == 0:
            return None
        a = self.s[0]
        self.s = self.s[1:]
        return a
    def back(self):
        if len(self.s) == 0:
            return None
        a = self.s[-1]
        self.s = self.s[:-1]
        return a
         
def opiter(s):
    global forwards
    global backwards
    forwards = 0
    backwards = 0
    def nxt():
        global forwards
        global backwards
        while forwards + backwards < len(s):
            a = s[forwards]
            forwards += 1
            yield a
        while True:
            yield None

    def bck():
        global forwards
        global backwards
        while forwards + backwards < len(s):
            a = s[len(s) - backwards - 1]
            backwards += 1
            yield a
        while True:
            yield None
    f = nxt()
    b = bck()

    return (lambda: next(f)), (lambda: next(b))


for _ in range(int(input())):
    n = int(input())
    s = input()

    nxt, back = opiter(s)
    a = nxt()
    b = back()
    operations = 0
    while not (a is None or b is None):
        print(a, b, operations)
        if a == b:
            operations += 1
            if operations > 300:
                print("-1")
                break
            if a == '0':
                pass
            else:
                pass
        else:
            a = nxt()
            b = back()
    if operations <= 300:
        print("yes")

