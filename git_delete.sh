#!/bin/bash

# サブディレクトリ内のすべての.gitディレクトリと.gitignoreファイルを削除する
remove_git_dirs_and_gitignore() {
    # カレントディレクトリ内のすべてのディレクトリを取得
    dirs=(*/)

    # 各サブディレクトリに対して処理を行う
    for dir in "${dirs[@]}"; do
        cd "$dir"

        # .gitディレクトリを削除
        if [ -d ".git" ]; then
            rm -rf .git
            echo "Removed .git directory from $(pwd)"
        fi

        # .gitignoreファイルを削除
        if [ -f ".gitignore" ]; then
            rm .gitignore
            echo "Removed .gitignore file from $(pwd)"
        fi

        # サブディレクトリに対して再帰的に関数を呼び出す
        remove_git_dirs_and_gitignore
        cd ..
    done
}

# メイン処理
remove_git_dirs_and_gitignore