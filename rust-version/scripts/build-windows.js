#!/usr/bin/env node

/**
 * Windows 平台打包脚本
 * 用于构建 Windows 版本的 Tauri 应用
 */

import { execSync } from 'child_process';
import { fileURLToPath } from 'url';
import { dirname, resolve } from 'path';
import { existsSync, mkdirSync, cpSync, readdirSync, statSync } from 'fs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const projectRoot = resolve(__dirname, '..');

// 颜色输出
const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  red: '\x1b[31m',
  cyan: '\x1b[36m'
};

function log(message, color = 'reset') {
  console.log(`${colors[color]}${message}${colors.reset}`);
}

function exec(command, options = {}) {
  const defaultOptions = {
    cwd: projectRoot,
    stdio: 'inherit',
    encoding: 'utf-8'
  };
  return execSync(command, { ...defaultOptions, ...options });
}

function getLatestFile(directory, pattern) {
  if (!existsSync(directory)) return null;
  
  const files = readdirSync(directory)
    .filter(f => pattern.test(f))
    .map(f => ({
      name: f,
      path: resolve(directory, f),
      stat: statSync(resolve(directory, f))
    }))
    .sort((a, b) => b.stat.mtime - a.stat.mtime);
  
  return files.length > 0 ? files[0] : null;
}

async function buildWindows() {
  log('========================================', 'cyan');
  log('  Windows 平台打包脚本', 'bright');
  log('========================================', 'cyan');
  console.log();

  // 1. 检查环境
  log('📋 检查打包环境...', 'yellow');
  try {
    exec('npm --version', { stdio: 'pipe' });
    exec('cargo --version', { stdio: 'pipe' });
    log('✓ 环境检查通过', 'green');
  } catch (error) {
    log('✗ 环境检查失败，请确保已安装 Node.js 和 Rust', 'red');
    process.exit(1);
  }
  console.log();

  // 2. 安装依赖
  log('📦 安装依赖...', 'yellow');
  try {
    exec('npm install');
    log('✓ 依赖安装完成', 'green');
  } catch (error) {
    log('✗ 依赖安装失败', 'red');
    process.exit(1);
  }
  console.log();

  // 3. 构建前端
  log('🔨 构建前端项目...', 'yellow');
  try {
    exec('npm run build');
    log('✓ 前端构建完成', 'green');
  } catch (error) {
    log('✗ 前端构建失败', 'red');
    process.exit(1);
  }
  console.log();

  // 4. 构建 Windows 应用
  log('🪟 开始构建 Windows 应用...', 'yellow');
  log('   目标平台: x86_64-pc-windows-msvc', 'cyan');
  console.log();

  try {
    // 使用 Tauri CLI 构建 Windows 版本
    exec('npm run tauri build -- --target x86_64-pc-windows-msvc');
    log('✓ Windows 应用构建完成', 'green');
  } catch (error) {
    log('✗ Windows 应用构建失败', 'red');
    log('   注意: 在 macOS/Linux 上交叉编译 Windows 需要额外配置', 'yellow');
    log('   建议使用 GitHub Actions 或在 Windows 环境中构建', 'yellow');
    process.exit(1);
  }
  console.log();

  // 5. 查找并显示输出文件
  log('📂 查找生成的安装包...', 'yellow');
  
  const outputDirs = [
    resolve(projectRoot, 'src-tauri/target/x86_64-pc-windows-msvc/release/bundle'),
    resolve(projectRoot, 'src-tauri/target/release/bundle')
  ];

  let foundFiles = [];
  
  for (const dir of outputDirs) {
    if (!existsSync(dir)) continue;
    
    // 查找 .msi 安装包
    const msiDir = resolve(dir, 'msi');
    if (existsSync(msiDir)) {
      const msiFile = getLatestFile(msiDir, /\.msi$/);
      if (msiFile) foundFiles.push({ type: 'MSI 安装包', ...msiFile });
    }
    
    // 查找 .exe 安装包 (NSIS)
    const nsisDir = resolve(dir, 'nsis');
    if (existsSync(nsisDir)) {
      const exeFile = getLatestFile(nsisDir, /\.exe$/);
      if (exeFile) foundFiles.push({ type: 'EXE 安装包', ...exeFile });
    }
    
    // 查找便携版 .exe
    const releaseDir = resolve(dir, '../');
    if (existsSync(releaseDir)) {
      const portableExe = getLatestFile(releaseDir, /\.exe$/);
      if (portableExe) foundFiles.push({ type: '便携版 EXE', ...portableExe });
    }
  }

  if (foundFiles.length > 0) {
    log('✓ 找到以下生成的文件:', 'green');
    console.log();
    foundFiles.forEach(file => {
      const sizeMB = (file.stat.size / 1024 / 1024).toFixed(2);
      log(`  [${file.type}]`, 'cyan');
      log(`    文件: ${file.name}`, 'bright');
      log(`    路径: ${file.path}`, 'reset');
      log(`    大小: ${sizeMB} MB`, 'reset');
      console.log();
    });
  } else {
    log('⚠ 未找到生成的安装包文件', 'yellow');
  }

  // 6. 完成
  log('========================================', 'cyan');
  log('  Windows 打包流程完成!', 'bright');
  log('========================================', 'cyan');
}

// 运行构建
buildWindows().catch(error => {
  log(`错误: ${error.message}`, 'red');
  process.exit(1);
});
