# 任务日历前端项目

基于 Vue 3 + TypeScript + Element Plus 的任务日历前端应用。

## 功能特性

- 🎯 **首页统计**：展示任务统计、甘特图、进度分析
- 📅 **日历视图**：月视图展示任务，支持日报/周报导出
- 📋 **任务看板**：三栏式拖拽任务管理，支持暂存队列
- 🏷️ **标签管理**：灵活的标签分类系统
- 🔐 **用户认证**：完整的登录/登出功能
- ⌨️ **快捷键**：Alt+Enter 快速添加任务

## 技术栈

- **框架**：Vue 3 + Composition API + TypeScript
- **路由**：Vue Router 4
- **状态管理**：Pinia
- **UI组件**：Element Plus
- **图表**：ECharts + Vue-ECharts
- **工具库**：VueUse、Day.js、Axios
- **构建工具**：Vite

## 项目结构

```
src/
├── components/     # 公共组件
├── views/          # 页面组件
│   ├── Home.vue    # 首页
│   ├── Calendar.vue # 日历页
│   ├── Tasks.vue   # 任务看板
│   ├── Tags.vue    # 标签管理
│   └── Login.vue   # 登录页
├── stores/         # 状态管理
│   └── user.ts     # 用户状态
├── router/         # 路由配置
├── layout/         # 布局组件
└── main.ts         # 入口文件
```

## 快速开始

### 环境要求

- Node.js >= 16.0.0
- npm >= 7.0.0

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
npm run dev
```

访问 http://localhost:3000

### 构建生产版本

```bash
npm run build
```

### 代码检查

```bash
npm run lint
```

## 页面功能说明

### 首页 (/home)
- 时间最长的5个任务展示
- 每日处理任务数统计图表
- 每日创建任务数统计图表
- 任务处理记录甘特图

### 日历页 (/calendar)
- 月视图展示任务分布
- 右键菜单导出日报/周报
- 月份翻页功能
- 任务数量徽章显示

### 任务看板 (/tasks)
- 三栏式布局：计划中、制作中、已完成
- 拖拽功能支持任务状态变更
- 暂存队列管理未分类任务
- 右键菜单：编辑、暂停、添加标签、删除
- 快捷键：Alt+Enter 快速添加任务

### 标签管理 (/tags)
- 标签列表展示
- 标签搜索功能
- 标签颜色管理
- 任务数量统计

## 演示账号

- 用户名：admin
- 密码：123456

## 开发说明

### 组件开发规范
- 使用 Composition API + `<script setup>`
- 类型定义使用 TypeScript
- 组件样式使用 Scoped CSS
- 遵循 Element Plus 设计规范

### 状态管理
- 使用 Pinia 进行状态管理
- 用户信息存储在 localStorage
- API 调用使用 Axios 封装

### 路由配置
- 使用 Vue Router 4
- 路由守卫保护需要登录的页面
- 路由懒加载优化性能

## 浏览器支持

- Chrome >= 88
- Firefox >= 78
- Safari >= 14
- Edge >= 88