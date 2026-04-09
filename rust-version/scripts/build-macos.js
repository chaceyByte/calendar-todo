#!/usr/bin/env node

/**
 * macOS 平台打包脚本
 * 用于构建 macOS 版本的 Tauri 应用
 */

import { execSync } from 'child_process';
import { fileURLToPath } from 'url';
import { dirname, resolve } from 'path';
import { existsSync, mkdirSync, readdirSync, statSync } from 'fs';

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

function execPipe(command, options = {}) {
  return execSync(command, {
    cwd: projectRoot,
    stdio: 'pipe',
    encoding: 'utf-8',
    ...options
  }).trim();
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

async function buildMacOS() {
  log('========================================', 'cyan');
  log('  macOS 平台打包脚本', 'bright');
  log('========================================', 'cyan');
  console.log();

  // 1. 检查环境
  log('📋 检查打包环境...', 'yellow');
  try {
    const nodeVersion = execPipe('node --version');
    log(`  ✓ Node.js: ${nodeVersion}`, 'green');
    
    const npmVersion = execPipe('npm --version');
    log(`  ✓ npm: ${npmVersion}`, 'green');
    
    const rustVersion = execPipe('rustc --version');
    log(`  ✓ Rust: ${rustVersion}`, 'green');
    
    const cargoVersion = execPipe('cargo --version');
    log(`  ✓ Cargo: ${cargoVersion}`, 'green');
    
    log('✓ 环境检查通过', 'green');
  } catch (error) {
    log('✗ 环境检查失败，请确保已安装 Node.js 和 Rust', 'red');
    process.exit(1);
  }
  console.log();

  // 2. 清理之前的构建（可选）
  log('🧹 清理之前的构建文件...', 'yellow');
  try {
    if (existsSync(resolve(projectRoot, 'dist'))) {
      exec('rm -rf dist');
    }
    log('✓ 清理完成', 'green');
  } catch (error) {
    log('⚠ 清理跳过', 'yellow');
  }
  console.log();

  // 3. 检查资源文件
  log('🎨 检查资源文件...', 'yellow');
  const iconsDir = resolve(projectRoot, 'src-tauri/icons');
  if (!existsSync(iconsDir)) {
    mkdirSync(iconsDir, { recursive: true });
  }
  
  // 检查并创建 macOS 图标
  const icnsPath = resolve(iconsDir, 'icon.icns');
  if (!existsSync(icnsPath)) {
    log('  📱 创建 macOS 图标...', 'cyan');
    try {
      const png32Path = resolve(iconsDir, '32x32.png');
      if (existsSync(png32Path)) {
        const iconsetDir = resolve(projectRoot, 'Icon.iconset');
        mkdirSync(iconsetDir, { recursive: true });
        
        // 生成各种尺寸的图标
        const sizes = [
          { size: 16, src: '32x32.png', out: 'icon_16x16.png' },
          { size: 32, src: '32x32.png', out: 'icon_16x16@2x.png' },
          { size: 32, src: '32x32.png', out: 'icon_32x32.png' },
          { size: 64, src: '32x32.png', out: 'icon_32x32@2x.png' },
          { size: 128, src: '32x32.png', out: 'icon_128x128.png' },
          { size: 256, src: '32x32.png', out: 'icon_128x128@2x.png' },
          { size: 256, src: '32x32.png', out: 'icon_256x256.png' },
          { size: 512, src: '32x32.png', out: 'icon_256x256@2x.png' },
          { size: 512, src: '32x32.png', out: 'icon_512x512.png' }
        ];
        
        for (const { size, src, out } of sizes) {
          try {
            execPipe(`sips -z ${size} ${size} ${resolve(iconsDir, src)} --out ${resolve(iconsetDir, out)}`);
          } catch (e) {
            // 如果 sips 失败，尝试使用其他方式
          }
        }
        
        // 创建 icns 文件
        try {
          execPipe(`iconutil -c icns ${iconsetDir} -o ${icnsPath}`);
          exec('rm -rf Icon.iconset');
          log('  ✓ macOS 图标创建完成', 'green');
        } catch (e) {
          log('  ⚠ 图标创建失败，将使用默认图标', 'yellow');
        }
      } else {
        log('  ⚠ 未找到源图标文件，将使用默认图标', 'yellow');
      }
    } catch (error) {
      log('  ⚠ 图标创建失败，将使用默认图标', 'yellow');
    }
  } else {
    log('  ✓ macOS 图标已存在', 'green');
  }
  console.log();

  // 4. 安装依赖
  log('📦 安装依赖...', 'yellow');
  try {
    exec('npm install --legacy-peer-deps');
    log('✓ 依赖安装完成', 'green');
  } catch (error) {
    log('✗ 依赖安装失败', 'red');
    process.exit(1);
  }
  console.log();

  // 5. 构建前端
  log('🔨 构建前端项目...', 'yellow');
  try {
    exec('npx vite build');
    log('✓ 前端构建完成', 'green');
  } catch (error) {
    log('⚠ 前端构建失败，尝试继续构建 Tauri', 'yellow');
  }
  console.log();

  // 6. 检查 Rust 代码
  log('🔧 检查 Rust 代码...', 'yellow');
  try {
    exec('cd src-tauri && cargo check');
    log('✓ Rust 代码检查通过', 'green');
  } catch (error) {
    log('✗ Rust 代码检查失败', 'red');
    log('💡 建议：尝试运行 cargo clean 后重新构建', 'yellow');
    process.exit(1);
  }
  console.log();

  // 7. 构建 Rust 后端
  log('📦 构建 Rust 后端...', 'yellow');
  try {
    exec('cd src-tauri && cargo build --release');
    log('✓ Rust 后端构建成功', 'green');
  } catch (error) {
    log('✗ Rust 构建失败', 'red');
    log('💡 尝试：', 'yellow');
    log('   1. cargo clean', 'yellow');
    log('   2. 手动检查 Cargo.toml 配置', 'yellow');
    log('   3. rustup update', 'yellow');
    process.exit(1);
  }
  console.log();

  // 8. 构建 macOS 应用
  log('🍎 构建 macOS 应用...', 'yellow');
  try {
    exec('npx tauri build');
    log('✓ macOS 应用构建完成', 'green');
  } catch (error) {
    log('⚠ Tauri CLI 构建失败', 'yellow');
    log('📦 但基础 Rust 应用已构建', 'cyan');
    log('💡 可以使用 cargo run --release 直接运行', 'cyan');
  }
  console.log();

  // 9. 查找并显示输出文件
  log('📂 查找生成的安装包...', 'yellow');
  
  const outputDirs = [
    resolve(projectRoot, 'src-tauri/target/release/bundle/macos'),
    resolve(projectRoot, 'src-tauri/target/release/bundle/dmg'),
    resolve(projectRoot, 'src-tauri/target/universal-apple-darwin/release/bundle/macos'),
    resolve(projectRoot, 'src-tauri/target/universal-apple-darwin/release/bundle/dmg'),
    resolve(projectRoot, 'src-tauri/target/aarch64-apple-darwin/release/bundle/macos'),
    resolve(projectRoot, 'src-tauri/target/aarch64-apple-darwin/release/bundle/dmg'),
    resolve(projectRoot, 'src-tauri/target/x86_64-apple-darwin/release/bundle/macos'),
    resolve(projectRoot, 'src-tauri/target/x86_64-apple-darwin/release/bundle/dmg')
  ];

  let foundFiles = [];
  
  for (const dir of outputDirs) {
    if (!existsSync(dir)) continue;
    
    // 查找 .app 应用包
    const appFile = getLatestFile(dir, /\.app$/);
    if (appFile) {
      // 计算 app 包大小
      try {
        const duOutput = execPipe(`du -sh "${appFile.path}"`);
        const size = duOutput.split('\t')[0];
        foundFiles.push({ type: 'macOS 应用 (.app)', ...appFile, sizeStr: size });
      } catch (e) {
        foundFiles.push({ type: 'macOS 应用 (.app)', ...appFile, sizeStr: '未知' });
      }
    }
    
    // 查找 .dmg 安装包
    const dmgFile = getLatestFile(dir, /\.dmg$/);
    if (dmgFile) {
      const sizeMB = (dmgFile.stat.size / 1024 / 1024).toFixed(2);
      foundFiles.push({ type: 'DMG 安装包', ...dmgFile, sizeStr: `${sizeMB} MB` });
    }
  }

  if (foundFiles.length > 0) {
    log('✓ 找到以下生成的文件:', 'green');
    console.log();
    foundFiles.forEach(file => {
      log(`  [${file.type}]`, 'cyan');
      log(`    文件: ${file.name}`, 'bright');
      log(`    路径: ${file.path}`, 'reset');
      log(`    大小: ${file.sizeStr}`, 'reset');
      console.log();
    });
  } else {
    // 检查是否有原始的可执行文件
    const releaseDir = resolve(projectRoot, 'src-tauri/target/release');
    if (existsSync(releaseDir)) {
      const binaryFile = getLatestFile(releaseDir, /^[^.]+$/); // 无扩展名的可执行文件
      if (binaryFile && binaryFile.stat.isFile() && binaryFile.stat.mode & 0o111) {
        log('✓ 找到可执行文件:', 'green');
        log(`  文件: ${binaryFile.name}`, 'bright');
        log(`  路径: ${binaryFile.path}`, 'reset');
        const sizeMB = (binaryFile.stat.size / 1024 / 1024).toFixed(2);
        log(`  大小: ${sizeMB} MB`, 'reset');
        console.log();
      }
    }
    log('⚠ 未找到标准的安装包文件', 'yellow');
  }

  // 10. 完成
  log('========================================', 'cyan');
  log('  macOS 打包流程完成!', 'bright');
  log('========================================', 'cyan');
  console.log();
  log('💡 提示:', 'cyan');
  log('   - .app 文件可以直接双击运行', 'reset');
  log('   - .dmg 文件是标准的 macOS 安装包', 'reset');
  log('   - 开发模式运行: npm run tauri dev', 'reset');
}

// 运行构建
buildMacOS().catch(error => {
  log(`错误: ${error.message}`, 'red');
  process.exit(1);
});
