# Number letter counts

I DIDNT COMPLETE THIS, THIS IS ANNOYING

def ans(n):
    refrence = {1: 3, 2: 3, 3: 4, 4: 4, 5: 4, 6: 3, 7: 5, 8: 5, 9: 4, 10: 3, 11: 6, 12: 6, 13: 8, 20: 6, 30: 6, 100: 7, 1000: 8}
    n = list(str(n))
    n = ['0']*(4-len(n))+n
    print(n)###
    result = 0
    if int(n[-2])==1: # 1*
        if int(n[-1])<4: # 10, 11, 12, 13
            look = int(n.pop(-2)+n.pop())
            result += refrence[int(look)]
        else: # 14 to 19
            look = int(n.pop())
            n.pop()
            result += 4+refrence[int(look)]
    elif int(n[-2])==0: # 0*
        if n[-1]==0:
            n.pop()
            n.pop()
        look = n.pop()
        n.pop()
        result += refrence[int(look)]
    else:
        result += refrence[int(n.pop(-2))] + 2 + refrence[int(n.pop())]
    return result

print(ans(56))