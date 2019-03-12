[English](https://github.com/kako-jun/gitp)

# :baby_bottle: gitp

[![Build Status](https://travis-ci.org/kako-jun/gitp.svg?branch=master)](https://travis-ci.org/kako-jun/gitp)

`gitp` は、シンプルなコマンドラインツールです

- 複数のリポジトリからの一括 `clone`、一括 `pull`、一括 `push`
- 面倒なコミットコメントの入力を省略

などを実現します

個人で多くのリポジトリを管理していて、「コミットコメントは固定でイイから、早く簡単に操作したい」場合に便利です

（`gitp` の `p` は Personal です）

Goで書かれているため、多くのOSで動作します

　

## Description

### Demo

![demo](https://raw.githubusercontent.com/kako-jun/gitp/master/assets/screen_1.gif)

### VS.

Gitは便利なツールです

コミットコメントには、後で他人が振り返りやすい言葉が求められています

一方、Gitの使い方は自由です

masterブランチしかないプライベートリポジトリなど、毎回丁寧なコミットコメントを書くのが過剰なケースもあります

　

【いかにもパーソナルユースな例えの絵。うがーー】

　

`gitp` を使わなかった場合の例

```sh
$ git add -A
$ git commit -m update.
$ git push origin master
```

`gitp` を使った場合の例

```sh
$ gitp push
```

　

行儀が悪い？

時間節約こそが優先の場合もあるんです

ほか、Gitコマンドを覚えたくないデザイナーに、Gitを使ってもらう場合にも有効かもしれません

（ほとんどの作業が、固定の1行で済みますから）

　

## Installation

### Requirements

- Operating System

    - Windows
    - macOS
    - Linux

- Dependency

    - [kako-jun/cdand](https://github.com/kako-jun/cdand)

### Download binaries

- Windows: [gitp.zip](https://github.com/kako-jun/gitp/raw/master/bin/windows/gitp.zip)
- macOS: [gitp.dmg](https://github.com/kako-jun/gitp/raw/master/bin/mac/gitp.dmg)
- Linux ( `chmod u+x gitp` required)

    - x64: [gitp_amd64.tar.gz](https://github.com/kako-jun/gitp/raw/master/bin/linux/gitp_amd64.tar.gz)
    - ARM: [gitp_arm64.tar.gz](https://github.com/kako-jun/gitp/raw/master/bin/linux/gitp_arm64.tar.gz)
    - Raspberry Pi: [gitp_armv7l.tar.gz](https://github.com/kako-jun/gitp/raw/master/bin/linux/gitp_armv7l.tar.gz)

### go get

```sh
$ go get github.com/kako-jun/gitp
```

　

## Features

### Usage

まず、`gitp_config.json` を作ります

`gitp init` で雛形が生成されますので、それを書き換えて以下のようにします

```js
{
    "comments": {
        "default": "update."
    },
    "user": {
        "name": "kako-jun",
        "email": "3541096+kako-jun@users.noreply.github.com"
    },
    "repos": [
        {
            "enabled": true,
            "name": "gist-exporter",
            "remotes": {
                "origin": {
                    "ssh": "git@ssh.dev.azure.com:v3/kako-jun/gist-exporter/gist-exporter",
                    "https": "https://kako-jun@dev.azure.com/kako-jun/gist-exporter/_git/gist-exporter"
                },
                "second": {
                    "ssh": "git@github.com:kako-jun/gist-exporter.git",
                    "https": "https://github.com/kako-jun/gist-exporter.git"
                }
            }
        },
        {
            "enabled": false,
            "name": "benford",
            "remotes": {
                "origin": {
                    "ssh": "git@github.com:kako-jun/benford.git"
                }
            }
        }
    ]
}
```

次に、`gitp clone` を実行します

TREE gitp_config.json

の状態になります

これで準備完了です

`gitp` は、このようにリポジトリのディレクトリが並んだ状態を想定しています

　

使い方は、大きく分けて3種類あります

1. 全てのリポジトリに対して、`gitp` のバッチコマンドを実行する
2. 1つのリポジトリに対して、`gitp` のバッチコマンドを実行する
3. 全てのリポジトリに対して、自由に `git` コマンドを実行する
4. 1つのリポジトリに対して、自由に `git` コマンドを実行する

#### 1. 全てのリポジトリに対して、`gitp` のバッチコマンドを実行する

```sh
$ gitp clone
$ gitp remote add
$ gitp config user
$ gitp pull
$ gitp push
```

最も使う機会が多いため、短くシンプルです

`gitp_config.json` 内で、`enabled` を `false` にしておいたリポジトリは、スキップされます

#### 2. 1つのリポジトリに対して、`gitp` のバッチコマンドを実行する

```sh
$ gitp clone [repository name]
$ gitp remote add [repository name]
$ gitp config user [repository name]
$ gitp pull [repository name]
$ gitp push [repository name]
```

`[repository name]` には、リポジトリ名（ディレクトリ名）を指定します

TABでの補完が効くため、高速に打てます

#### 3. 全てのリポジトリに対して、自由に `git` コマンドを実行する

```sh
$ gitp -a clone hoge
$ gitp -a pull origin master
$ gitp -a add -A
$ gitp -a commit -m update.
$ gitp -a push origin master
$ gitp -a remote add public
...
```

これは便利です

1の使い方では大雑把すぎる場合は、この使い方で対応しましょう

#### 4. 1つのリポジトリに対して、自由に `git` コマンドを実行する

```sh
$ gitp [repository name] clone hoge
$ gitp [repository name] pull origin master
$ gitp [repository name] add -A
$ gitp [repository name] commit -m update.
$ gitp [repository name] push origin master
$ gitp [repository name] remote add public
...
```

`git` コマンドを実行するのと大して変わりませんが、「サブディレクトリに `cd` しなくて良い」という利点があります

　

「なぜ `gitp` が便利なのか……？」の例を、以下に挙げます

#### Examples

##### e.g. 複数のリポジトリの変更状態をまとめて確認できる

```sh
$ gitp -a git status
```

で可能です

##### e.g. 複数のリポジトリを一括リセットできる

```sh
$ gitp -a checkout .
```

で可能です

##### e.g. `gitp pull` だけでも相当便利

```sh
$ gitp push
```

を使うとコミットコメントが固定になるため、それがイヤな場合は `gitp pull` だけを使いましょう

##### e.g. グローバルな設定を汚さない

`gitp` は、各リポジトリ内の `.git/config` しか変更しません

　

#### Unsupported

##### https より ssh が優先される

`gitp_config.json` で、リモートリポジトリのURLとして `ssh`、`https` の両方を書いた場合は、`ssh` が優先されます

鍵を登録していない場合、`clone` に失敗するでしょう

`https` で接続したい場合は、`ssh` を空にするか、`ssh` ごと消してください

##### リポジトリ名が `gitp` のバッチコマンドと重複してはいけない

`clone`、`remote`、`config`、`pull`、`push` などのリポジトリ名は使えません

　

### Coding

```golang
import "github.com/kako-jun/gitp/gitp-core"

gitp.Exec(gitpCommand, allRepo, repo, gitCommandAndArgs...)
```

### Contributing

Pull Requestを歓迎します

- `gitp` をより便利にする機能の追加
- より洗練されたGoでの書き方
- バグの発見、修正
- もっと良い英訳、日本語訳があると教えたい

など、アイデアを教えてください

　

## Authors

kako-jun

- :octocat: https://github.com/kako-jun
- :notebook: https://gist.github.com/kako-jun
- :house: https://llll-ll.com
- :bird: https://twitter.com/kako_jun_42

### :lemon: Lemonade stand

寄付を頂けたら、少し豪華な猫エサを買おうと思います

下のリンクから、Amazonギフト券（Eメールタイプ）を送ってください

「受取人」欄には `kako.hydrajin@gmail.com` と入力してください

　**[:hearts: Donate](https://www.amazon.co.jp/Amazon%E3%82%AE%E3%83%95%E3%83%88%E5%88%B8-1_JP_Email-Amazon%E3%82%AE%E3%83%95%E3%83%88%E5%88%B8-E%E3%83%A1%E3%83%BC%E3%83%AB%E3%82%BF%E3%82%A4%E3%83%97-Amazon%E3%83%99%E3%83%BC%E3%82%B7%E3%83%83%E3%82%AF/dp/B004N3APGO/)**

- 「メッセージ」欄を使って、感想を伝えることもできます
- 送り主が誰かは分かりません
- ¥15 から送れます

　

## License

This project is licensed under the MIT License.

See the [LICENSE](https://github.com/kako-jun/gitp/blob/master/LICENSE) file for details.

## Acknowledgments

- [Go](https://golang.org/)
- and you
