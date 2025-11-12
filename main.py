import sys
import hashlib
import random
import csv

class RandPosition:
    def __init__(self, reserved_data, width, height, s, mod):
        self.reserved_data = reserved_data
        self.width = width
        self.heigth = height
        self.s = s
        self.mod = mod

        self.nomal_seat_data = [i+1 for i in range(45)]
        self.nomal_seat_data.remove(20)

        # 座席の初期化
        #   確定席 -> number, 未定積 -> -1, 席がない場所 -> 0 
        self.class_position = [[-1 for j in range(width)] for i in range(height)]
        self.class_position[height-1][0] = 0
        self.class_position[height-1][1] = 0
        self.class_position[height-1][4] = 0

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

    # ハッシュ値の剰余を取り数値化
    def hash_mod(self, h):
        hash_int = int(h, 16) % self.mod
        return hash_int

    # 指定席の人を配置＆重複がないかなどの確認
    def position_reserved(self):
        for num, h, w in self.reserved_data:
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
        output_data = [["" if item == 0 else str(item) for item in row] for row in self.class_position[::-1]]
        for i in output_data:
            print(*i, sep="\t")
            # print(*i)
        output_file_name = "output_data.csv" 
        with open(output_file_name, "w", newline="", encoding="utf-8") as f:
            writer = csv.writer(f)
            writer.writerows(output_data)

s = "atcoder"  # seed値
mod = 1e9  # mod

width = 5  # 横幅
height = 9  # 縦幅

# 指定席データ  dataformat -> [num, h, w]
reserved_seat_data = []

filename = "./reserved_seat.csv"
with open(filename, "r", newline="", encoding="utf-8") as f:
    reader = csv.reader(f)
    marker = next(reader)
    for row in reader:
        print(f"指定席データ: {row}")
        reserved_seat_data.append(list(map(int, row)))

solve = RandPosition(reserved_seat_data, width, height, s, mod)
solve.solve()
