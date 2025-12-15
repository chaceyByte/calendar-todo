package com.taskcalendar.util;

import javax.imageio.ImageIO;
import java.awt.*;
import java.awt.image.BufferedImage;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.Base64;
import java.util.Random;

public class CaptchaUtil {
    
    private static final String CHARS = "ABCDEFGHJKMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz23456789";
    private static final int WIDTH = 120;
    private static final int HEIGHT = 40;
    private static final int CODE_LENGTH = 4;
    private static final Random random = new Random();
    
    /**
     * 生成验证码
     * @return 返回包含验证码文本和Base64图片字符串的数组
     */
    public static String[] generateCaptcha() {
        // 创建图像
        BufferedImage image = new BufferedImage(WIDTH, HEIGHT, BufferedImage.TYPE_INT_RGB);
        Graphics g = image.getGraphics();
        
        // 设置背景色
        g.setColor(Color.WHITE);
        g.fillRect(0, 0, WIDTH, HEIGHT);
        
        // 生成随机验证码
        StringBuilder code = new StringBuilder();
        for (int i = 0; i < CODE_LENGTH; i++) {
            code.append(CHARS.charAt(random.nextInt(CHARS.length())));
        }
        
        // 绘制验证码
        g.setFont(new Font("Arial", Font.BOLD, 20));
        for (int i = 0; i < code.length(); i++) {
            // 随机颜色
            g.setColor(new Color(
                random.nextInt(150),
                random.nextInt(150),
                random.nextInt(150)
            ));
            // 随机位置
            g.drawString(
                String.valueOf(code.charAt(i)),
                15 + i * 25,
                20 + random.nextInt(10)
            );
        }
        
        // 绘制干扰线
        for (int i = 0; i < 5; i++) {
            g.setColor(new Color(
                random.nextInt(150),
                random.nextInt(150),
                random.nextInt(150)
            ));
            g.drawLine(
                random.nextInt(WIDTH),
                random.nextInt(HEIGHT),
                random.nextInt(WIDTH),
                random.nextInt(HEIGHT)
            );
        }
        
        // 绘制干扰点
        for (int i = 0; i < 30; i++) {
            g.setColor(new Color(
                random.nextInt(150),
                random.nextInt(150),
                random.nextInt(150)
            ));
            g.drawOval(
                random.nextInt(WIDTH),
                random.nextInt(HEIGHT),
                1,
                1
            );
        }
        
        g.dispose();
        
        // 将图像转换为Base64字符串
        ByteArrayOutputStream baos = new ByteArrayOutputStream();
        try {
            ImageIO.write(image, "PNG", baos);
        } catch (IOException e) {
            throw new RuntimeException("生成验证码失败", e);
        }
        
        byte[] imageBytes = baos.toByteArray();
        String base64Image = Base64.getEncoder().encodeToString(imageBytes);
        
        return new String[]{code.toString(), base64Image};
    }
}