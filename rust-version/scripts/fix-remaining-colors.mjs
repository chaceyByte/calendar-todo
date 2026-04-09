#!/usr/bin/env node

/**
 * 修复剩余颜色脚本
 * 处理 Calendar.vue 和 Tasks.vue 中遗漏的硬编码颜色
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// 额外的颜色映射（脚本遗漏的）
const additionalColorMap = {
  // Calendar.vue 和 Tasks.vue 中的特殊颜色
  '#f5f7fa': 'var(--bg-page)',
  '#e8ecf1': 'var(--bg-input)',
  '#fafbfc': 'var(--bg-card)',
  '#475569': 'var(--text-secondary)',
  '#3b82f6': 'var(--color-primary)',
  '#2563eb': 'var(--color-primary-hover)',
  '#8b5cf6': '#8b5cf6', // 紫色保持
  '#047857': 'var(--color-success)',
  '#c2410c': '#c2410c', // 橙色保持
  '#ea580c': '#ea580c', // 橙色保持
  '#d97706': '#d97706', // 黄色保持
  '#7c3aed': '#7c3aed', // 紫色保持
  '#ecfdf5': 'var(--color-success-bg)',
  '#d1fae5': 'var(--color-success-bg)',
  '#ffedd5': 'var(--color-warning-bg)',
  '#c4c4c7': 'var(--text-tertiary)',
  
  // 透明色
  'rgba(255, 255, 255, 0.6)': 'var(--glass-bg)',
  'rgba(255, 255, 255, 0.3)': 'rgba(255, 255, 255, 0.3)',
  'rgba(255, 255, 255, 0.2)': 'rgba(255, 255, 255, 0.2)',
  'rgba(255, 255, 255, 0.25)': 'rgba(255, 255, 255, 0.25)',
  'rgba(59, 130, 246, 0.1)': 'var(--color-primary-light)',
  'rgba(59, 130, 246, 0.15)': 'var(--color-primary-light)',
  'rgba(59, 130, 246, 0.18)': 'var(--color-primary-light)',
  'rgba(16, 185, 129, 0.15)': 'var(--color-success-bg)',
  'rgba(16, 185, 129, 0.18)': 'var(--color-success-bg)',
  'rgba(245, 158, 11, 0.1)': 'var(--color-warning-bg)',
  'rgba(245, 158, 11, 0.15)': 'var(--color-warning-bg)',
  'rgba(245, 158, 11, 0.18)': 'var(--color-warning-bg)',
  'rgba(139, 92, 246, 0.15)': 'rgba(139, 92, 246, 0.15)',
  'rgba(139, 92, 246, 0.18)': 'rgba(139, 92, 246, 0.18)',
  'rgba(102, 126, 234, 0.05)': 'var(--bg-hover)',
  'rgba(0, 102, 204, 0.02)': 'var(--bg-hover)',
  'rgba(239, 68, 68, 0.15)': 'var(--color-error-bg)',
  'rgba(0, 0, 0, 0.2)': 'rgba(0, 0, 0, 0.2)',
};

// 需要处理的文件
const targetFiles = [
  'src/views/Calendar.vue',
  'src/views/Tasks.vue',
];

// 替换文件中的颜色
function replaceColorsInFile(filePath) {
  const fullPath = path.join(path.join(__dirname, '..'), filePath);
  let content = fs.readFileSync(fullPath, 'utf-8');
  let modified = false;
  let replacements = [];
  
  // 按长度降序排序
  const sortedColors = Object.keys(additionalColorMap).sort((a, b) => b.length - a.length);
  
  for (const color of sortedColors) {
    const variable = additionalColorMap[color];
    // 跳过不需要替换的（值和键相同）
    if (variable === color) continue;
    
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
    fs.writeFileSync(fullPath, content, 'utf-8');
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
  console.log('🔧 修复剩余颜色...\n');
  
  const results = [];
  
  for (const file of targetFiles) {
    const result = replaceColorsInFile(file);
    if (result) {
      results.push(result);
    }
  }
  
  // 输出结果
  console.log('='.repeat(60));
  console.log('✅ 修复完成！\n');
  
  if (results.length === 0) {
    console.log('没有需要修复的颜色。');
    return;
  }
  
  console.log(`📊 共处理 ${results.length} 个文件:\n`);
  
  for (const result of results) {
    console.log(`📝 ${result.file}`);
    for (const r of result.replacements) {
      console.log(`   ${r}`);
    }
    console.log('');
  }
  
  console.log('='.repeat(60));
}

main();
