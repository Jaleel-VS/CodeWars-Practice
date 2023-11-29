def beeramid(bonus, price):
    if bonus < 0 or price <= 0:
        return 0

    total_cost = 0
    cans = 0
    levels = 0

    while total_cost <= bonus:
        levels += 1
        cans += levels ** 2
        total_cost = cans * price

    return levels - 1 if levels else 0

