#!/usr/bin/env node

/**
 * 主题迁移脚本
 * 自动将 Vue 组件中的硬编码颜色替换为 CSS 变量
 */

const fs = require('fs');
const path = require('path');

// 颜色映射表：硬编码颜色 -> CSS 变量
const colorMap = {
  // 背景色
  '#f7f9fb': 'var(--bg-page)',
  '#ffffff': 'var(--bg-card)',
  '#f2f4f6': 'var(--bg-input)',
  '#f5f5f7': 'var(--bg-card-hover)',
  '#f8fafc': 'var(--bg-card)',
  '#f1f5f9': 'var(--bg-input)',
  '#e2e8f0': 'var(--border-color)',
  '#e3e3e8': 'var(--border-light)',
  
  // 文字色
  '#0f172a': 'var(--text-primary)',
  '#1d1d1f': 'var(--text-primary)',
  '#1a1a2e': 'var(--text-primary)',
  '#64748b': 'var(--text-secondary)',
  '#86868b': 'var(--text-secondary)',
  '#6b7280': 'var(--text-secondary)',
  '#5a6a7a': 'var(--text-secondary)',
  '#8b9aa8': 'var(--text-secondary)',
  '#94a3b8': 'var(--text-tertiary)',
  '#a1a1a6': 'var(--text-tertiary)',
  
  // 主色调
  '#0058be': 'var(--color-primary)',
  '#0071e3': 'var(--color-primary)',
  '#0066cc': 'var(--color-primary)',
  '#0052a3': 'var(--color-primary-hover)',
  
  // 边框
  '#cbd5e1': 'var(--border-color)',
  '#e0e3e5': 'var(--scrollbar-thumb)',
  '#c2c6d6': 'var(--scrollbar-thumb-hover)',
  
  // 功能色
  '#34d399': 'var(--color-success)',
  '#10b981': 'var(--color-success)',
  '#059669': 'var(--color-success)',
  '#f59e0b': 'var(--color-warning)',
  '#ff9500': 'var(--color-warning)',
  '#ef4444': 'var(--color-error)',
  '#ff3b30': 'var(--color-error)',
  
  // 透明背景
  'rgba(0, 0, 0, 0.05)': 'var(--shadow-sm)',
  'rgba(0, 0, 0, 0.08)': 'var(--border-color)',
  'rgba(0, 0, 0, 0.04)': 'var(--bg-hover)',
  'rgba(0, 0, 0, 0.06)': 'var(--divider-color)',
  'rgba(0, 102, 204, 0.08)': 'var(--color-primary-light)',
  'rgba(0, 102, 204, 0.12)': 'var(--color-primary-light)',
  'rgba(0, 102, 204, 0.3)': 'rgba(0, 113, 227, 0.3)',
  'rgba(255, 255, 255, 0.8)': 'var(--glass-bg)',
  
  // 白色和黑色
  'white': 'var(--bg-card)',
  '#fff': 'var(--bg-card)',
  'black': 'var(--text-primary)',
  '#000': 'var(--text-primary)',
  '#000000': 'var(--text-primary)',
};

// 需要跳过的文件
const skipFiles = [
  'theme.css',
  'theme.ts',
  'GeneralSettingsContent.vue',
  'Sidebar.vue',
  'Settings.vue',
];

// 需要处理的目录
const targetDirs = [
  'src/components',
  'src/views',
];

// 递归获取所有 Vue 文件
function getVueFiles(dir, files = []) {
  const items = fs.readdirSync(dir);
  
  for (const item of items) {
    const fullPath = path.join(dir, item);
    const stat = fs.statSync(fullPath);
    
    if (stat.isDirectory()) {
      getVueFiles(fullPath, files);
    } else if (item.endsWith('.vue') && !skipFiles.includes(item)) {
      files.push(fullPath);
    }
  }
  
  return files;
}

// 替换文件中的颜色
function replaceColorsInFile(filePath) {
  let content = fs.readFileSync(filePath, 'utf-8');
  let modified = false;
  let replacements = [];
  
  // 按长度降序排序，避免短颜色替换影响长颜色
  const sortedColors = Object.keys(colorMap).sort((a, b) => b.length - a.length);
  
  for (const color of sortedColors) {
    const variable = colorMap[color];
    const regex = new RegExp(escapeRegExp(color), 'g');
    
    if (regex.test(content)) {
      const matches = content.match(regex);
      if (matches) {
        content = content.replace(regex, variable);
        replacements.push(`${color} -> ${variable} (${matches.length}处)`);
        modified = true;
      }
    }
  }
  
  if (modified) {
    fs.writeFileSync(filePath, content, 'utf-8');
    return { file: filePath, replacements };
  }
  
  return null;
}

// 转义正则特殊字符
function escapeRegExp(string) {
  return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

// 主函数
function main() {
  console.log('🎨 开始主题迁移...\n');
  
  const baseDir = path.join(__dirname, '..');
  const results = [];
  
  for (const dir of targetDirs) {
    const fullDir = path.join(baseDir, dir);
    if (!fs.existsSync(fullDir)) {
      console.log(`⚠️  目录不存在: ${dir}`);
      continue;
    }
    
    const files = getVueFiles(fullDir);
    console.log(`📁 扫描 ${dir}: 找到 ${files.length} 个文件`);
    
    for (const file of files) {
      const result = replaceColorsInFile(file);
      if (result) {
        results.push(result);
      }
    }
  }
  
  // 输出结果
  console.log('\n' + '='.repeat(60));
  console.log('✅ 迁移完成！\n');
  
  if (results.length === 0) {
    console.log('没有找到需要替换的颜色。');
    return;
  }
  
  console.log(`📊 共处理 ${results.length} 个文件:\n`);
  
  for (const result of results) {
    const relativePath = path.relative(baseDir, result.file);
    console.log(`📝 ${relativePath}`);
    for (const r of result.replacements) {
      console.log(`   ${r}`);
    }
    console.log('');
  }
  
  console.log('='.repeat(60));
  console.log('\n💡 提示：请检查替换后的文件，确保没有误替换。');
  console.log('   特别关注：图片路径、注释中的颜色代码等。');
}

main();
