import request from '@/utils/request'

export interface Tag {
  id: number
  name: string
  color?: string
}

// 标签相关的API函数
export const createTag = async (tag: Tag) => {
  try {
    const response = await request.post('/api/tags', tag)
    return response.data || response
  } catch (error) {
    console.error('创建标签失败:', error)
    throw error
  }
}

export const updateTag = async (id: number, updatedTag: Tag) => {
  try {
    const response = await request.put(`/api/tags/${id}`, updatedTag)
    return response.data || response
  } catch (error) {
    console.error('更新标签失败:', error)
    throw error
  }
}

export const deleteTag = async (id: number) => {
  try {
    await request.delete(`/api/tags/${id}`)
  } catch (error) {
    console.error('删除标签失败:', error)
    throw error
  }
}

export const getTags = async () => {
  try {
    const response = await request.get('/api/tags')
    return response.data || response
  } catch (error) {
    console.error('获取标签列表失败:', error)
    throw error
  }
}