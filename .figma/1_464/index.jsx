import React from 'react';

import styles from './index.module.scss';

const Component = () => {
  return (
    <div className={styles.frame}>
      <div className={styles.alertModalSimulatedC}>
        <div className={styles.background2}>
          <div className={styles.overlayShadow}>
            <div className={styles.container2}>
              <div className={styles.background} />
              <div className={styles.container}>
                <div className={styles.heading4}>
                  <p className={styles.text}>无法删除标签</p>
                </div>
                <p className={styles.text2}>该标签当前正在被使用中</p>
              </div>
            </div>
            <div className={styles.container3}>
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
            <div className={styles.container4}>
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
        <div className={styles.headerTopNavBarShare}>
          <div className={styles.container7}>
            <div className={styles.input}>
              <div className={styles.container5}>
                <p className={styles.text9}>搜索标签...</p>
              </div>
              <img
                src="../image/mnfdq8ct-yef8ul4.svg"
                className={styles.container6}
              />
            </div>
          </div>
          <div className={styles.container10}>
            <div className={styles.button3}>
              <img
                src="../image/mnfdq8ct-gwds038.svg"
                className={styles.container8}
              />
            </div>
            <div className={styles.button4}>
              <img
                src="../image/mnfdq8ct-d3igqwb.svg"
                className={styles.container9}
              />
            </div>
            <div className={styles.button4}>
              <img
                src="../image/mnfdq8ct-vsmslzz.svg"
                className={styles.container9}
              />
            </div>
            <div className={styles.margin}>
              <div className={styles.overlayShadow2}>
                <img
                  src="../image/mnfdq8cy-orj44xn.png"
                  className={styles.userAvatar}
                />
              </div>
            </div>
          </div>
        </div>
        <div className={styles.asideSideNavBarShare}>
          <div className={styles.container14}>
            <div className={styles.background3}>
              <div className={styles.overlayShadow3}>
                <img
                  src="../image/mnfdq8ct-jz1l4gm.svg"
                  className={styles.container11}
                />
              </div>
            </div>
            <div className={styles.container13}>
              <p className={styles.text10}>Sanctuary</p>
              <div className={styles.container12}>
                <p className={styles.text11}>Deep Work Mode</p>
              </div>
            </div>
          </div>
          <div className={styles.nav}>
            <div className={styles.link}>
              <img
                src="../image/mnfdq8ct-ugr6qc1.svg"
                className={styles.container15}
              />
              <p className={styles.text12}>Dashboard</p>
            </div>
            <div className={styles.link2}>
              <img
                src="../image/mnfdq8ct-8cbu1oh.svg"
                className={styles.container9}
              />
              <p className={styles.text13}>Tasks</p>
            </div>
            <div className={styles.link3}>
              <img
                src="../image/mnfdq8ct-e6zrcit.svg"
                className={styles.container9}
              />
              <p className={styles.text14}>标签管理</p>
              <div className={styles.backgroundShadow} />
            </div>
            <div className={styles.link4}>
              <img
                src="../image/mnfdq8ct-qovjhcb.svg"
                className={styles.container16}
              />
              <p className={styles.text15}>Calendar</p>
            </div>
            <div className={styles.link5}>
              <img
                src="../image/mnfdq8ct-vu61j4x.svg"
                className={styles.container17}
              />
              <p className={styles.text16}>Analytics</p>
            </div>
          </div>
          <div className={styles.margin2}>
            <div className={styles.button5}>
              <div className={styles.buttonShadow2}>
                <img
                  src="../image/mnfdq8ct-n528uwr.svg"
                  className={styles.container18}
                />
                <p className={styles.text17}>New Task</p>
              </div>
            </div>
          </div>
          <div className={styles.container19}>
            <div className={styles.link6}>
              <img
                src="../image/mnfdq8ct-d3igqwb.svg"
                className={styles.container9}
              />
              <p className={styles.text18}>Settings</p>
            </div>
            <div className={styles.link7}>
              <img
                src="../image/mnfdq8ct-qd26mre.svg"
                className={styles.container15}
              />
              <p className={styles.text19}>Log Out</p>
            </div>
          </div>
        </div>
      </div>
      <div className={styles.container42}>
        <div className={styles.headerSection}>
          <div className={styles.container20}>
            <div className={styles.heading2}>
              <p className={styles.text20}>标签管理</p>
            </div>
            <p className={styles.text21}>整理和归类您的认知资产</p>
          </div>
          <div className={styles.button6}>
            <img
              src="../image/mnfdq8ct-axxogy1.svg"
              className={styles.container9}
            />
            <p className={styles.text22}>创建新标签</p>
          </div>
        </div>
        <div className={styles.tagsBentoGrid}>
          <div className={styles.autoWrapper}>
            <div className={styles.tagCardHighPriority}>
              <div className={styles.container21}>
                <div className={styles.background4}>
                  <p className={styles.text23}>URGENT</p>
                </div>
              </div>
              <div className={styles.heading3}>
                <p className={styles.text24}>高优先级</p>
              </div>
              <div className={styles.container23}>
                <p className={styles.text25}>24 个任务使用中</p>
                <div className={styles.container22}>
                  <div className={styles.backgroundBorder} />
                  <div className={styles.backgroundBorder2} />
                  <div className={styles.backgroundBorder3}>
                    <p className={styles.text26}>+12</p>
                  </div>
                </div>
              </div>
            </div>
            <div className={styles.tagCardLearning}>
              <div className={styles.container24}>
                <div className={styles.background5}>
                  <p className={styles.text27}>LEARNING</p>
                </div>
              </div>
              <div className={styles.heading3}>
                <p className={styles.text24}>持续学习</p>
              </div>
              <div className={styles.container25}>
                <p className={styles.text28}>7 个任务使用中</p>
              </div>
            </div>
          </div>
          <div className={styles.autoWrapper2}>
            <div className={styles.tagCardDeepWork}>
              <div className={styles.container26}>
                <div className={styles.background6}>
                  <p className={styles.text29}>FOCUS</p>
                </div>
              </div>
              <div className={styles.heading3}>
                <p className={styles.text24}>深度工作</p>
              </div>
              <div className={styles.container28}>
                <p className={styles.text30}>18 个任务使用中</p>
                <div className={styles.container27}>
                  <div className={styles.backgroundBorder4} />
                  <div className={styles.margin3}>
                    <div className={styles.backgroundBorder5} />
                  </div>
                </div>
              </div>
            </div>
            <div className={styles.tagCardArchivableUnu}>
              <div className={styles.container30}>
                <div className={styles.background7}>
                  <p className={styles.text31}>UNUSED</p>
                </div>
                <div className={styles.button7}>
                  <img
                    src="../image/mnfdq8ct-gyf6eoj.svg"
                    className={styles.container29}
                  />
                </div>
              </div>
              <div className={styles.heading32}>
                <p className={styles.text32}>待分类</p>
              </div>
              <div className={styles.container31}>
                <p className={styles.text33}>无任务使用 - 可安全删除</p>
              </div>
            </div>
          </div>
          <div className={styles.autoWrapper3}>
            <div className={styles.tagCardPersonal}>
              <div className={styles.container32}>
                <div className={styles.background8}>
                  <p className={styles.text34}>PERSONAL</p>
                </div>
              </div>
              <div className={styles.heading33}>
                <p className={styles.text24}>个人生活</p>
              </div>
              <div className={styles.container33}>
                <p className={styles.text25}>42 个任务使用中</p>
              </div>
            </div>
            <div className={styles.emptyStateAddCard}>
              <div className={styles.margin4}>
                <div className={styles.background9}>
                  <img
                    src="../image/mnfdq8ct-th88gnt.svg"
                    className={styles.container34}
                  />
                </div>
              </div>
              <p className={styles.text35}>定义新维度</p>
            </div>
          </div>
        </div>
        <div className={styles.footerUsageAnalytics}>
          <div className={styles.container40}>
            <div className={styles.container37}>
              <div className={styles.container35}>
                <p className={styles.text36}>标签总数</p>
              </div>
              <div className={styles.container36}>
                <p className={styles.text37}>12</p>
              </div>
            </div>
            <div className={styles.container39}>
              <div className={styles.container38}>
                <p className={styles.text38}>最活跃标签</p>
              </div>
              <p className={styles.text39}>深度工作</p>
            </div>
          </div>
          <div className={styles.container41}>
            <img
              src="../image/mnfdq8cy-ga3i96d.png"
              className={styles.analyticsTrend}
            />
          </div>
        </div>
      </div>
    </div>
  );
}

export default Component;
