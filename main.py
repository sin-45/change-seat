import sys
import hashlib
import random

class RandPosition:
    def __init__(self, reserved_data, width, height, s, mod):
        self.reserved_data = reserved_data
        self.width = width
        self.heigth = height
        self.s = s
        self.mod = mod

        self.nomal_seat_data = [i+1 for i in range(45)]

        # 座席の初期化
        #   確定席 -> number, 未定積 -> -1, 席がない場所 -> 0 
        self.class_position = [[-1 for j in range(width)] for i in range(height)]
        self.class_position[height-1][0] = 0
        self.class_position[height-1][1] = 0

    def solve(self):
        self.position_reserved()
        self.rand_seed()
        self.position_set()
        self.output()
    
    # seed値決定＆shuffle
    def rand_seed(self):
        h = self.hash_sha256()
        hash_int = self.hash_mod(h)
        random.seed(hash_int)
        random.shuffle(self.nomal_seat_data)

    # ハッシュ値生成
    def hash_sha256(self):
        x = str(self.s)
        hash_result = hashlib.sha256(x.encode('utf-8')).hexdigest()
        return hash_result

    # ハッシュ値を数値化
    def hash_mod(self, h):
        hash_int = int(h, 16) % self.mod
        return hash_int

    # 指定席の人を配置＆重複がないかなどの確認
    def position_reserved(self):
        for num, h, w in reserved_seat_data:
            if num not in self.nomal_seat_data:
                print("Error: 指定が重複しています")
                sys.exit()
            self.nomal_seat_data.remove(num)

        for num, h, w in self.reserved_data:
            if self.position_ok(h, w):
                self.class_position[h][w] = num
            else:
                print("Error: 座席指定がかぶっています")
                sys.exit()

    # 指定席でない人を配置
    def position_set(self):
        cnt = 0
        for h in range(self.heigth):
            for w in range(self.width):
                if self.position_ok(h, w):
                    self.class_position[h][w] = self.nomal_seat_data[cnt]
                    cnt += 1

    # 座席が空いているかの確認
    def position_ok(self, h, w):
        if self.class_position[h][w] == -1:
            return True
        else:
            return False

    # 出力
    def output(self):
        for i in self.class_position[::-1]:
            print(*i, sep="\t")
            # print(*i)

s = "AtCoder"  # seed値
mod = 1e9  # mod

width = 5  # 横幅
height = 9  # 縦幅

# 指定席データ  dataformat -> [num, h, w]
reserved_seat_data = []

solve = RandPosition(reserved_seat_data, width, height, s, mod)
solve.solve()
