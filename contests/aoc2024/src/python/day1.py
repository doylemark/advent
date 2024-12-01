import sys

D = open(sys.argv[1]).read().strip()

lines = D.splitlines()

LL = []
RR = []

for line in lines:
    L, R = line.split()
    LL.append(int(L))
    RR.append(int(R))

LL = sorted(LL)
RR = sorted(RR)

ans = 0

for (l, r) in zip(LL, RR):
    ans += abs(l - r)

print(ans)

map = {}
for n in RR:
    map[n] = map.get(n, 0) + 1

sum = 0
for n in LL:
    sum += n * map.get(n, 0)

print(sum)
