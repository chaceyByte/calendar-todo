#!/bin/bash

# 同时启动 Vite dev server 和 Tauri

echo "🚀 启动开发环境..."

# 启动 Vite dev server（在后台）
echo "📦 启动 Vite dev server..."
npm run dev &
VITE_PID=$!

# 等待 Vite 启动
echo "⏳ 等待 Vite 启动..."
sleep 3

# 启动 Tauri
echo "🎯 启动 Tauri..."
npx tauri dev

# 清理
echo "🧹 清理进程..."./
kill $VITE_PID 2>/dev/null
