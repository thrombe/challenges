# Counting Sundays
'''
1 Jan 1900 was a Monday.
A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
'''

def ans():
    day = 1
    year = 1899
    score = 0
    for month in range(1, 12*101+1):
        month = month % 12 # month 0 is december
        if month == 1: year += 1
        if month in [1, 3, 5, 7, 8, 10, 0]: day += 31
        elif month == 2:
            if year % 4 == 0:
                if year % 100 == 0 and year % 400 != 0: day += 28
                else: day += 29
            else: day += 28
        else: day += 30
        if 1901 <= year <= 2000:
            if day % 7 == 0: score += 1
    print(score, month, year)

def ans2(): # i think this one is just coincidance
    print(12*100 // 7)

ans()