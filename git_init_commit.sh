#!/bin/bash

# カレントディレクトリ直下のすべてのディレクトリを取得
dirs=(*/)

# 各ディレクトリを走査
for dir in "${dirs[@]}"; do
    # ディレクトリ名から最後の "/" を削除
    dir=${dir%/}

    # .gitディレクトリが存在するかチェック
    if [ -d "$dir/.git" ]; then
        # サブディレクトリに移動
        pushd "$dir" > /dev/null

        # ステージングエリアにすべてのファイルを追加
        git add .

        # イニシャルコミットを作成
        git commit -m "Initial commit" --allow-empty

        # 元のディレクトリに戻る
        popd > /dev/null
    fi
done