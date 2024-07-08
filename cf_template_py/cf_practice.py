from collections import defaultdict
import sys
def cf1883F():
    n = (int(input())) # tc数量
    for _ in range(n):
        _ = (int(input()))
        arr = (list(map(int,input().split())))
        last_occur_mp = defaultdict(int)
        for index,num in enumerate(arr):
            last_occur_mp[num] = index
        ans = 0
        fisrt_occur_st = defaultdict(bool)
        rest = len(last_occur_mp)
        for index,num in enumerate(arr):
            if num not in fisrt_occur_st:
                fisrt_occur_st[num] = True
                ans += rest
            if last_occur_mp[num] == index:
                rest -= 1
        sys.stdout.write(str(ans)+"\n")