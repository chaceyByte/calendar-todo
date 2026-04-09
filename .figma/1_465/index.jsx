import React from 'react';

import styles from './index.module.scss';

const Component = () => {
  return (
    <div className={styles.alertModalSimulatedC}>
      <div className={styles.background2}>
        <div className={styles.overlayShadow}>
          <div className={styles.container3}>
            <div className={styles.background}>
              <img
                src="../image/mnfdqhzx-1ssh053.svg"
                className={styles.container}
              />
            </div>
            <div className={styles.container2}>
              <div className={styles.heading4}>
                <p className={styles.text}>无法删除标签</p>
              </div>
              <p className={styles.text2}>该标签当前正在被使用中</p>
            </div>
          </div>
          <div className={styles.container4}>
            <p className={styles.text6}>
              <span className={styles.text3}>标签&nbsp;</span>
              <span className={styles.text4}>“高优先级”</span>
              <span className={styles.text3}>&nbsp;已关联到&nbsp;</span>
              <span className={styles.text5}>24</span>
              <span className={styles.text3}>
                &nbsp;个进行中的任务。在删除此标签
                <br />
                前，请先移除或更换这些任务的标签属性。
              </span>
            </p>
          </div>
          <div className={styles.container5}>
            <div className={styles.button}>
              <p className={styles.text7}>取消</p>
            </div>
            <div className={styles.button2}>
              <div className={styles.buttonShadow}>
                <p className={styles.text8}>查看相关任务</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default Component;
