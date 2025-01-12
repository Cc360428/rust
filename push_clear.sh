#!/bin/bash

# 遍历所有目录，删除名为 target 的目录
find . -type d -name "target" -exec rm -rf {} +

# 添加更改到 git
git add .