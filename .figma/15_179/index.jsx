import React from 'react';

import styles from './index.module.scss';

const Component = () => {
  return (
    <div className={styles.createEditModalOverl}>
      <div className={styles.backgroundShadow}>
        <div className={styles.container2}>
          <p className={styles.text}>新建标签</p>
          <div className={styles.button}>
            <img src="../image/mnffqlf8-hgor95a.svg" className={styles.container} />
          </div>
        </div>
        <div className={styles.container6}>
          <div className={styles.inputField}>
            <p className={styles.text2}>标签名称</p>
            <div className={styles.container3}>
              <p className={styles.text3}>新标签项目</p>
            </div>
          </div>
          <div className={styles.colorPicker}>
            <p className={styles.text4}>选择识别色</p>
            <div className={styles.container5}>
              <div className={styles.autoWrapper}>
                <div className={styles.button2}>
                  <div className={styles.buttonShadow} />
                </div>
                <div className={styles.button3}>
                  <img
                    src="../image/mnffqlf9-otzwov4.svg"
                    className={styles.container4}
                  />
                </div>
              </div>
              <div className={styles.button4} />
              <div className={styles.button5} />
              <div className={styles.button6} />
              <div className={styles.button7} />
              <div className={styles.button8} />
              <div className={styles.button9} />
            </div>
          </div>
          <div className={styles.footerActions}>
            <div className={styles.button10}>
              <p className={styles.text5}>取消</p>
            </div>
            <div className={styles.button11}>
              <div className={styles.buttonShadow2}>
                <p className={styles.text6}>保存标签</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default Component;
