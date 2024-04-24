#!/bin/bash

# 現在のディレクトリから開始して、すべての.gitディレクトリを削除する
remove_git_dirs() {
    # カレントディレクトリ内のすべてのディレクトリを取得
    dirs=(*/)

    # カレントディレクトリ内の.gitディレクトリを削除
    if [ -d ".git" ]; then
        rm -rf .git
        echo "Removed .git directory from $(pwd)"
    fi

    # 各サブディレクトリに対して再帰的に関数を呼び出す
    for dir in "${dirs[@]}"; do
        cd "$dir"
        remove_git_dirs
        cd ..
    done
}

# メイン処理
remove_git_dirs