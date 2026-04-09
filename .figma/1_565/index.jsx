import React from 'react';

import styles from './index.module.scss';

const Component = () => {
  return (
    <div className={styles.container24}>
      <div className={styles.headerSection}>
        <div className={styles.container}>
          <div className={styles.heading2}>
            <p className={styles.text}>标签管理</p>
          </div>
          <p className={styles.text2}>整理和归类您的认知资产</p>
        </div>
        <div className={styles.button}>
          <img src="../image/mnfdqhzt-rb35uie.svg" className={styles.container2} />
          <p className={styles.text3}>创建新标签</p>
        </div>
      </div>
      <div className={styles.tagsBentoGrid}>
        <div className={styles.autoWrapper}>
          <div className={styles.tagCardHighPriority}>
            <div className={styles.container3}>
              <div className={styles.background}>
                <p className={styles.text4}>URGENT</p>
              </div>
            </div>
            <div className={styles.heading3}>
              <p className={styles.text5}>高优先级</p>
            </div>
            <div className={styles.container5}>
              <p className={styles.text6}>24 个任务使用中</p>
              <div className={styles.container4}>
                <div className={styles.backgroundBorder} />
                <div className={styles.backgroundBorder2} />
                <div className={styles.backgroundBorder3}>
                  <p className={styles.text7}>+12</p>
                </div>
              </div>
            </div>
          </div>
          <div className={styles.tagCardLearning}>
            <div className={styles.container6}>
              <div className={styles.background2}>
                <p className={styles.text8}>LEARNING</p>
              </div>
            </div>
            <div className={styles.heading3}>
              <p className={styles.text5}>持续学习</p>
            </div>
            <div className={styles.container7}>
              <p className={styles.text9}>7 个任务使用中</p>
            </div>
          </div>
        </div>
        <div className={styles.autoWrapper2}>
          <div className={styles.tagCardDeepWork}>
            <div className={styles.container8}>
              <div className={styles.background3}>
                <p className={styles.text10}>FOCUS</p>
              </div>
            </div>
            <div className={styles.heading3}>
              <p className={styles.text5}>深度工作</p>
            </div>
            <div className={styles.container10}>
              <p className={styles.text11}>18 个任务使用中</p>
              <div className={styles.container9}>
                <div className={styles.backgroundBorder4} />
                <div className={styles.margin}>
                  <div className={styles.backgroundBorder5} />
                </div>
              </div>
            </div>
          </div>
          <div className={styles.tagCardArchivableUnu}>
            <div className={styles.container12}>
              <div className={styles.background4}>
                <p className={styles.text12}>UNUSED</p>
              </div>
              <div className={styles.button2}>
                <img
                  src="../image/mnfdqhzt-khrqlu6.svg"
                  className={styles.container11}
                />
              </div>
            </div>
            <div className={styles.heading32}>
              <p className={styles.text13}>待分类</p>
            </div>
            <div className={styles.container13}>
              <p className={styles.text14}>无任务使用 - 可安全删除</p>
            </div>
          </div>
        </div>
        <div className={styles.autoWrapper3}>
          <div className={styles.tagCardPersonal}>
            <div className={styles.container14}>
              <div className={styles.background5}>
                <p className={styles.text15}>PERSONAL</p>
              </div>
            </div>
            <div className={styles.heading33}>
              <p className={styles.text5}>个人生活</p>
            </div>
            <div className={styles.container15}>
              <p className={styles.text6}>42 个任务使用中</p>
            </div>
          </div>
          <div className={styles.emptyStateAddCard}>
            <div className={styles.margin2}>
              <div className={styles.background6}>
                <img
                  src="../image/mnfdqhzt-d3fh7yp.svg"
                  className={styles.container16}
                />
              </div>
            </div>
            <p className={styles.text16}>定义新维度</p>
          </div>
        </div>
      </div>
      <div className={styles.footerUsageAnalytics}>
        <div className={styles.container22}>
          <div className={styles.container19}>
            <div className={styles.container17}>
              <p className={styles.text17}>标签总数</p>
            </div>
            <div className={styles.container18}>
              <p className={styles.text18}>12</p>
            </div>
          </div>
          <div className={styles.container21}>
            <div className={styles.container20}>
              <p className={styles.text19}>最活跃标签</p>
            </div>
            <p className={styles.text20}>深度工作</p>
          </div>
        </div>
        <div className={styles.container23}>
          <img
            src="../image/mnfdqhzv-g5ny2vp.png"
            className={styles.analyticsTrend}
          />
        </div>
      </div>
    </div>
  );
}

export default Component;
