# probe

外部ロガー

実際にプログラムを動かす環境とデバッグ環境が異なり、高頻度の情報が必要なときにはデータの収集を外に持ち出したい。
CEDECで[prontf拡張の話](https://www.famitsu.com/news/202009/06205314.html)があった。
またその参照元である[FINAL FANTASY XVの開発を支えるバックエンド](https://cedec.cesa.or.jp/2016/session/ENG/4999.html)でログを分離する仕組みやそのフォーマットについても言及がある
基本はこの思想に基づいている


## Specification 

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

#### 実装例

```
LogBuilger("EntryA")
    .Tag("Type", "Engeneer")
    .Log("Name", "Mataro")
    .Log("Age", 35);
LogBuilger("EntryB")
    .Tag("Type", "Engeneer")
    .Log("Name", "Ingimar")
    .Log("Age", 36);
```

メモリ内にはKey-Valueな独自フォーマットで保持される

ID|Type|Data
:--|:--|:--
1|Start|EntryA
2|Start|EntryB
1|Tag|Type,Engeneer
1|Log|Name,Mataro
2|Tag|Type,Engeneer
2|Log|Name,Ingimar
2|Log|Age,36
1|Log|Age,35
1|Finish|Timestamp
2|Finish|Timestamp

1セッション単位でログを管理
ログ毎にUniqueなIDがあるのでAggregatorがログを再構築してCSVで保存

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


## Components

必要なもの

- Log Storage: ログを保存し呼び出せる
- Log Receiver: クライアントからの接続を受け付ける
- Log Client


### API

logを実装する

可変長可変型をどのように実装するのか? -> Enumでベタ実装

1. マクロ
1. Heterogeneous List (e.g. [frunk](https://github.com/lloydmeta/frunk))
1. Enumで型を定義

前2つは今のスキル的にやや難しいのでEnumで実現させる

## TODO

- [ ] format logAPIの実装
    - [x] primitive KeyValue
    - [ ] 構造体
        - derive macro実装が必要? serdeを参考にする
- [ ] ログ保存と閲覧CLIの作成
    - [ ] セッション別ファイルに保存する




## Memo

### macro

[Reference](https://doc.rust-jp.rs/book-ja/ch19-06-macros.html)
Rustの場合可変長引数は関数に渡せないためmacroを使う必要が有る

1. 宣言的マクロ `macro_rules!` を使い、マクロ式を展開してRustのコードが生成される
1. 手続き型マクロ コードを入力として受取、コードに作用する
    - deriveマクロ
    - 属性風マクロ
    - 関数風マクロ