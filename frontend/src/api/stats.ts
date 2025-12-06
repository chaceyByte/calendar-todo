import request from '@/utils/request';

/**
 * 获取时间占用最长的前5个任务
 */
export const getTopTimeConsumingTasks = () => {
  return request.get('/api/activities/stats/top-time-consuming');
};

/**
 * 获取最近14天每日处理的任务数量
 */
export const getDailyProcessedTasks = (days = 14) => {
  return request.get(`/api/activities/stats/daily-processed?days=${days}`);
};

/**
 * 获取最近14天每日创建的任务数量
 */
export const getDailyCreatedTasks = (days = 14) => {
  return request.get(`/api/activities/stats/daily-created?days=${days}`);
};

/**
 * 获取按标签分类的任务甘特图数据
 */
export const getGanttChartByTags = () => {
  return request.get('/api/activities/stats/gantt-by-tags');
};