# probe

外部ロガー

実際にプログラムを動かす環境とデバッグ環境が異なり、高頻度の情報が必要なときにはデータの収集を外に持ち出したい。
CEDECで[prontf拡張の話](https://www.famitsu.com/news/202009/06205314.html)があった。
基本はこの思想に基づいている


## Spevification 

### format

構造化されたログ

1. 通常のログと同じようにLevelを持つ
1. ログの分類ができるようにカテゴリを持つ
1. log messageをもつ(template構文が使える)
1. 埋め込まれたvalueのkey,value,typeを持つ

#### 実装例

```
LOG_INFO(
    "Battle.Player.HP", // Category
    "HP: {hp} / {hp_max}", // Message
    "hp", 12, // Key-Value
    "hp_max", 30
)
```

これが構造化される

```json
{
    "category": "Battle.Player.HP",
    "message": "HP: {hp} / {hp_max}",
    "timestamp": "",
    "threadname": "main thread",
    "values": {
        "hp": {
            "type": "int",
            "value": 12
        },
        "hp": {
            "type": "int",
            "value": 12
        },
    }
}

```

#### Level

- Debug
- Info
- Warning
- Error
- Fatal

#### Category

"."で階層を区切る`<cat1>.<cat2>`



## 方針

MessagePackを使う


### MessagePack

[MessagePack](https://msgpack.org/ja.html)
効率の良いバイナリ形式のオブジェクトシリアライズフォーマット。
