import request from '@/utils/request'

/**
 * 获取验证码
 * @returns 返回验证码图片的Base64编码
 */
export const getCaptcha = async () => {
  try {
    const response = await request.get('/api/captcha/generate')
    return response.data.image
  } catch (error) {
    console.error('获取验证码失败:', error)
    throw error
  }
}