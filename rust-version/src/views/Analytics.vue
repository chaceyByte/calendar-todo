<template>
  <div class="analytics-page">
    <!-- Header -->
    <header class="page-header">
      <div class="header-title">
        <p class="subtitle">Performance Overview</p>
        <h1 class="title">数据统计总结</h1>
      </div>
      <div class="header-actions">
        <div class="user-avatars">
          <div class="avatar-stack">
            <div class="avatar avatar-placeholder"></div>
            <div class="avatar-more">+4</div>
          </div>
        </div>
        <button class="share-btn">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="18" cy="5" r="3"/>
            <circle cx="6" cy="12" r="3"/>
            <circle cx="18" cy="19" r="3"/>
            <line x1="8.59" y1="13.51" x2="15.42" y2="17.49"/>
            <line x1="15.41" y1="6.51" x2="8.59" y2="10.49"/>
          </svg>
        </button>
      </div>
    </header>

    <!-- Main Grid -->
    <div class="analytics-grid">
      <!-- Top Row -->
      <div class="grid-row">
        <!-- Left Column: Stats Cards -->
        <div class="stats-column">
          <!-- Total Tasks Card -->
          <div class="stat-card">
            <div class="card-header">
              <div class="icon-wrapper blue">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
                  <polyline points="22 4 12 14.01 9 11.01"/>
                </svg>
              </div>
              <span class="trend up">+12%</span>
            </div>
            <div class="card-body">
              <p class="label">累计完成任务</p>
              <p class="value">1,284</p>
            </div>
            <div class="card-footer">
              <div class="progress-bar">
                <div class="progress-fill" style="width: 78%"></div>
              </div>
              <span class="progress-text">78% 完成率</span>
            </div>
          </div>

          <!-- Efficiency Card -->
          <div class="stat-card">
            <div class="card-header">
              <div class="icon-wrapper green">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="12" cy="12" r="10"/>
                  <polyline points="12 6 12 12 16 14"/>
                </svg>
              </div>
              <span class="trend up">+5.3%</span>
            </div>
            <div class="card-body">
              <p class="label">平均任务耗时</p>
              <p class="value">4.2<span class="unit">h</span></p>
            </div>
            <div class="card-footer">
              <div class="progress-bar">
                <div class="progress-fill green" style="width: 65%"></div>
              </div>
              <span class="progress-text">效率提升 15%</span>
            </div>
          </div>

          <!-- Completion Rate Card -->
          <div class="stat-card">
            <div class="card-header">
              <div class="icon-wrapper purple">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"/>
                </svg>
              </div>
              <span class="trend down">-2.1%</span>
            </div>
            <div class="card-body">
              <p class="label">本周完成率</p>
              <p class="value">89.3<span class="unit">%</span></p>
            </div>
            <div class="card-footer">
              <div class="progress-bar">
                <div class="progress-fill purple" style="width: 89%"></div>
              </div>
              <span class="progress-text">目标: 95%</span>
            </div>
          </div>

          <!-- Focus Time Card -->
          <div class="stat-card">
            <div class="card-header">
              <div class="icon-wrapper orange">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M12 2a10 10 0 1 0 10 10H12V2z"/>
                  <path d="M12 2a10 10 0 0 1 10 10"/>
                  <path d="M12 12L2.5 12"/>
                </svg>
              </div>
              <span class="trend up">+8.7%</span>
            </div>
            <div class="card-body">
              <p class="label">专注时长</p>
              <p class="value">32.5<span class="unit">h</span></p>
            </div>
            <div class="card-footer">
              <div class="progress-bar">
                <div class="progress-fill orange" style="width: 72%"></div>
              </div>
              <span class="progress-text">日均 4.6h</span>
            </div>
          </div>
        </div>

        <!-- Right Column: Charts -->
        <div class="charts-column">
          <!-- Task Distribution Chart -->
          <div class="chart-card">
            <div class="chart-header">
              <h3>任务分布</h3>
              <div class="legend">
                <span class="legend-item"><span class="dot blue"></span>已完成</span>
                <span class="legend-item"><span class="dot orange"></span>进行中</span>
                <span class="legend-item"><span class="dot gray"></span>待处理</span>
              </div>
            </div>
            <div class="chart-body">
              <div class="donut-chart">
                <svg viewBox="0 0 200 200" class="donut">
                  <!-- Background circle -->
                  <circle cx="100" cy="100" r="80" fill="none" stroke="#e8ecf1" stroke-width="20"/>
                  <!-- Completed segment (45%) -->
                  <circle cx="100" cy="100" r="80" fill="none" stroke="#3b82f6" stroke-width="20"
                          stroke-dasharray="226 502" stroke-dashoffset="0" transform="rotate(-90 100 100)"/>
                  <!-- In progress segment (30%) -->
                  <circle cx="100" cy="100" r="80" fill="none" stroke="var(--color-warning)" stroke-width="20"
                          stroke-dasharray="150 502" stroke-dashoffset="-226" transform="rotate(-90 100 100)"/>
                  <!-- Pending segment (25%) -->
                  <circle cx="100" cy="100" r="80" fill="none" stroke="#d1d5db" stroke-width="20"
                          stroke-dasharray="126 502" stroke-dashoffset="-376" transform="rotate(-90 100 100)"/>
                </svg>
                <div class="chart-center">
                  <span class="center-value">284</span>
                  <span class="center-label">总任务</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Weekly Trend Chart -->
          <div class="chart-card">
            <div class="chart-header">
              <h3>每周趋势</h3>
              <div class="time-selector">
                <button class="time-btn active">周</button>
                <button class="time-btn">月</button>
              </div>
            </div>
            <div class="chart-body">
              <div class="bar-chart">
                <div class="bar-group">
                  <div class="bar-stack">
                    <div class="bar-segment completed" style="height: 60%"></div>
                    <div class="bar-segment pending" style="height: 25%"></div>
                  </div>
                  <span class="bar-label">周一</span>
                </div>
                <div class="bar-group">
                  <div class="bar-stack">
                    <div class="bar-segment completed" style="height: 75%"></div>
                    <div class="bar-segment pending" style="height: 15%"></div>
                  </div>
                  <span class="bar-label">周二</span>
                </div>
                <div class="bar-group">
                  <div class="bar-stack">
                    <div class="bar-segment completed" style="height: 45%"></div>
                    <div class="bar-segment pending" style="height: 35%"></div>
                  </div>
                  <span class="bar-label">周三</span>
                </div>
                <div class="bar-group">
                  <div class="bar-stack">
                    <div class="bar-segment completed" style="height: 80%"></div>
                    <div class="bar-segment pending" style="height: 10%"></div>
                  </div>
                  <span class="bar-label">周四</span>
                </div>
                <div class="bar-group">
                  <div class="bar-stack">
                    <div class="bar-segment completed" style="height: 55%"></div>
                    <div class="bar-segment pending" style="height: 30%"></div>
                  </div>
                  <span class="bar-label">周五</span>
                </div>
                <div class="bar-group">
                  <div class="bar-stack">
                    <div class="bar-segment completed" style="height: 30%"></div>
                    <div class="bar-segment pending" style="height: 40%"></div>
                  </div>
                  <span class="bar-label">周六</span>
                </div>
                <div class="bar-group">
                  <div class="bar-stack">
                    <div class="bar-segment completed" style="height: 20%"></div>
                    <div class="bar-segment pending" style="height: 25%"></div>
                  </div>
                  <span class="bar-label">周日</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Bottom Row -->
      <div class="grid-row bottom">
        <!-- Quadrant Analysis -->
        <div class="quadrant-card">
          <div class="card-header-row">
            <h3>四象限分析</h3>
            <button class="more-btn">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="1"/>
                <circle cx="19" cy="12" r="1"/>
                <circle cx="5" cy="12" r="1"/>
              </svg>
            </button>
          </div>
          <div class="quadrant-grid">
            <div class="quadrant-item important-urgent">
              <div class="quadrant-header">
                <span class="quadrant-label">重要且紧急</span>
                <span class="quadrant-count">12</span>
              </div>
              <div class="quadrant-bar">
                <div class="quadrant-fill" style="width: 85%"></div>
              </div>
              <p class="quadrant-desc">需立即处理的任务</p>
            </div>
            <div class="quadrant-item important">
              <div class="quadrant-header">
                <span class="quadrant-label">重要不紧急</span>
                <span class="quadrant-count">28</span>
              </div>
              <div class="quadrant-bar">
                <div class="quadrant-fill" style="width: 65%"></div>
              </div>
              <p class="quadrant-desc">计划安排的任务</p>
            </div>
            <div class="quadrant-item urgent">
              <div class="quadrant-header">
                <span class="quadrant-label">紧急不重要</span>
                <span class="quadrant-count">8</span>
              </div>
              <div class="quadrant-bar">
                <div class="quadrant-fill" style="width: 40%"></div>
              </div>
              <p class="quadrant-desc">可委托他人的任务</p>
            </div>
            <div class="quadrant-item normal">
              <div class="quadrant-header">
                <span class="quadrant-label">不重要不紧急</span>
                <span class="quadrant-count">5</span>
              </div>
              <div class="quadrant-bar">
                <div class="quadrant-fill" style="width: 20%"></div>
              </div>
              <p class="quadrant-desc">需减少的任务</p>
            </div>
          </div>
        </div>

        <!-- Top Tags -->
        <div class="tags-card">
          <div class="card-header-row">
            <h3>热门标签</h3>
            <button class="more-btn">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="1"/>
                <circle cx="19" cy="12" r="1"/>
                <circle cx="5" cy="12" r="1"/>
              </svg>
            </button>
          </div>
          <div class="tags-list">
            <div class="tag-item">
              <div class="tag-info">
                <span class="tag-dot" style="background: #3b82f6"></span>
                <span class="tag-name">开发</span>
              </div>
              <div class="tag-stats">
                <span class="tag-count">45</span>
                <div class="tag-bar">
                  <div class="tag-fill" style="width: 90%; background: #3b82f6"></div>
                </div>
              </div>
            </div>
            <div class="tag-item">
              <div class="tag-info">
                <span class="tag-dot" style="background: var(--color-success)"></span>
                <span class="tag-name">设计</span>
              </div>
              <div class="tag-stats">
                <span class="tag-count">32</span>
                <div class="tag-bar">
                  <div class="tag-fill" style="width: 64%; background: var(--color-success)"></div>
                </div>
              </div>
            </div>
            <div class="tag-item">
              <div class="tag-info">
                <span class="tag-dot" style="background: var(--color-warning)"></span>
                <span class="tag-name">会议</span>
              </div>
              <div class="tag-stats">
                <span class="tag-count">28</span>
                <div class="tag-bar">
                  <div class="tag-fill" style="width: 56%; background: var(--color-warning)"></div>
                </div>
              </div>
            </div>
            <div class="tag-item">
              <div class="tag-info">
                <span class="tag-dot" style="background: #8b5cf6"></span>
                <span class="tag-name">文档</span>
              </div>
              <div class="tag-stats">
                <span class="tag-count">21</span>
                <div class="tag-bar">
                  <div class="tag-fill" style="width: 42%; background: #8b5cf6"></div>
                </div>
              </div>
            </div>
            <div class="tag-item">
              <div class="tag-info">
                <span class="tag-dot" style="background: var(--color-error)"></span>
                <span class="tag-name">紧急</span>
              </div>
              <div class="tag-stats">
                <span class="tag-count">15</span>
                <div class="tag-bar">
                  <div class="tag-fill" style="width: 30%; background: var(--color-error)"></div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">


</script>
<style scoped>
.analytics-page {
  padding: 32px 40px;
  min-height: 100vh;
  background: var(--bg-page);
}

/* Header */
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 32px;
}

.header-title .subtitle {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 1px;
  margin: 0 0 8px 0;
}

.header-title .title {
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 16px;
}

.avatar-stack {
  display: flex;
  align-items: center;
}

.avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  border: 2px solid var(--bg-card);
  object-fit: cover;
}

.avatar-placeholder {
  background: linear-gradient(135deg, #e8ecf1, #d0d7de);
}

.avatar-more {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: #f3f4f6;
  border: 2px solid var(--bg-card);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
  margin-left: -8px;
}

.share-btn {
  width: 44px;
  height: 44px;
  background: var(--bg-card);
  border: 1px solid #e8ecf1;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s ease;
}

.share-btn:hover {
  background: var(--bg-page);
  border-color: #d0d7de;
}

.share-btn svg {
  width: 20px;
  height: 20px;
  color: var(--text-secondary);
}

/* Grid Layout */
.analytics-grid {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.grid-row {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 24px;
}

.grid-row.bottom {
  grid-template-columns: 2fr 1fr;
}

/* Stats Column */
.stats-column {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 20px;
}

.stat-card {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 20px;
  box-shadow: 0 1px 3px var(--bg-hover);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.icon-wrapper {
  width: 44px;
  height: 44px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.icon-wrapper.blue {
  background: rgba(59, 130, 246, 0.1);
  color: #3b82f6;
}

.icon-wrapper.green {
  background: rgba(16, 185, 129, 0.1);
  color: var(--color-success);
}

.icon-wrapper.purple {
  background: rgba(139, 92, 246, 0.1);
  color: #8b5cf6;
}

.icon-wrapper.orange {
  background: rgba(245, 158, 11, 0.1);
  color: var(--color-warning);
}

.icon-wrapper svg {
  width: 22px;
  height: 22px;
}

.trend {
  font-size: 13px;
  font-weight: 600;
  padding: 4px 10px;
  border-radius: 20px;
}

.trend.up {
  color: var(--color-success);
  background: rgba(16, 185, 129, 0.1);
}

.trend.down {
  color: var(--color-error);
  background: rgba(239, 68, 68, 0.1);
}

.card-body .label {
  font-size: 13px;
  color: var(--text-secondary);
  margin: 0 0 8px 0;
}

.card-body .value {
  font-size: 32px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
}

.card-body .unit {
  font-size: 18px;
  font-weight: 500;
  color: var(--text-secondary);
  margin-left: 4px;
}

.card-footer {
  margin-top: 16px;
}

.progress-bar {
  height: 6px;
  background: #e8ecf1;
  border-radius: 3px;
  overflow: hidden;
  margin-bottom: 8px;
}

.progress-fill {
  height: 100%;
  background: #3b82f6;
  border-radius: 3px;
  transition: width 0.3s ease;
}

.progress-fill.green {
  background: var(--color-success);
}

.progress-fill.purple {
  background: #8b5cf6;
}

.progress-fill.orange {
  background: var(--color-warning);
}

.progress-text {
  font-size: 12px;
  color: var(--text-secondary);
}

/* Charts Column */
.charts-column {
  grid-column: span 2;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;
}

.chart-card {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 24px;
  box-shadow: 0 1px 3px var(--bg-hover);
}

.chart-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.chart-header h3 {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.legend {
  display: flex;
  gap: 16px;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: var(--text-secondary);
}

.legend-item .dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}

.legend-item .dot.blue {
  background: #3b82f6;
}

.legend-item .dot.orange {
  background: var(--color-warning);
}

.legend-item .dot.gray {
  background: #d1d5db;
}

.time-selector {
  display: flex;
  background: #f3f4f6;
  border-radius: 8px;
  padding: 4px;
}

.time-btn {
  padding: 6px 16px;
  background: transparent;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.time-btn.active {
  background: var(--bg-card);
  color: var(--text-primary);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.chart-body {
  display: flex;
  align-items: center;
  justify-content: center;
}

/* Donut Chart */
.donut-chart {
  position: relative;
  width: 200px;
  height: 200px;
}

.donut {
  width: 100%;
  height: 100%;
  transform: rotate(-90deg);
}

.chart-center {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  text-align: center;
}

.center-value {
  display: block;
  font-size: 32px;
  font-weight: 700;
  color: var(--text-primary);
}

.center-label {
  display: block;
  font-size: 13px;
  color: var(--text-secondary);
}

/* Bar Chart */
.bar-chart {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  width: 100%;
  height: 180px;
  padding: 0 8px;
}

.bar-group {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}

.bar-stack {
  display: flex;
  flex-direction: column-reverse;
  width: 32px;
  height: 140px;
  background: #f3f4f6;
  border-radius: 6px;
  overflow: hidden;
}

.bar-segment {
  width: 100%;
  transition: height 0.3s ease;
}

.bar-segment.completed {
  background: #3b82f6;
}

.bar-segment.pending {
  background: #d1d5db;
}

.bar-label {
  font-size: 12px;
  color: var(--text-secondary);
}

/* Bottom Section */
.quadrant-card,
.tags-card {
  background: var(--bg-card);
  border-radius: 16px;
  padding: 24px;
  box-shadow: 0 1px 3px var(--bg-hover);
}

.card-header-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.card-header-row h3 {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.more-btn {
  width: 32px;
  height: 32px;
  background: transparent;
  border: none;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s ease;
}

.more-btn:hover {
  background: #f3f4f6;
}

.more-btn svg {
  width: 18px;
  height: 18px;
  color: var(--text-secondary);
}

/* Quadrant Grid */
.quadrant-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
}

.quadrant-item {
  padding: 16px;
  background: var(--bg-page);
  border-radius: 12px;
}

.quadrant-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.quadrant-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
}

.quadrant-count {
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
}

.quadrant-bar {
  height: 6px;
  background: #e8ecf1;
  border-radius: 3px;
  overflow: hidden;
  margin-bottom: 8px;
}

.quadrant-fill {
  height: 100%;
  border-radius: 3px;
  transition: width 0.3s ease;
}

.quadrant-item.important-urgent .quadrant-fill {
  background: var(--color-error);
}

.quadrant-item.important .quadrant-fill {
  background: var(--color-warning);
}

.quadrant-item.urgent .quadrant-fill {
  background: #3b82f6;
}

.quadrant-item.normal .quadrant-fill {
  background: var(--color-success);
}

.quadrant-desc {
  font-size: 12px;
  color: var(--text-secondary);
  margin: 0;
}

/* Tags List */
.tags-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.tag-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.tag-info {
  display: flex;
  align-items: center;
  gap: 10px;
}

.tag-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
}

.tag-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.tag-stats {
  display: flex;
  align-items: center;
  gap: 12px;
}

.tag-count {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-secondary);
  min-width: 24px;
  text-align: right;
}

.tag-bar {
  width: 80px;
  height: 6px;
  background: #e8ecf1;
  border-radius: 3px;
  overflow: hidden;
}

.tag-fill {
  height: 100%;
  border-radius: 3px;
  transition: width 0.3s ease;
}
</style>
