import React from 'react';

import styles from './index.module.scss';

const Component = () => {
  return (
    <div className={styles.frame}>
      <div className={styles.asideSidebarNavigati}>
        <div className={styles.container3}>
          <div className={styles.background}>
            <div className={styles.overlayShadow} />
            <p className={styles.text}>shrine</p>
          </div>
          <div className={styles.container2}>
            <p className={styles.text2}>Sanctuary</p>
            <div className={styles.container}>
              <p className={styles.text3}>Deep Work Mode</p>
            </div>
          </div>
        </div>
        <div className={styles.nav}>
          <div className={styles.link}>
            <img
              src="../image/mnfd9x5v-2j6rojq.svg"
              className={styles.container4}
            />
            <p className={styles.text4}>Dashboard</p>
          </div>
          <div className={styles.link2}>
            <img
              src="../image/mnfd9x5v-uwxwwrn.svg"
              className={styles.container5}
            />
            <p className={styles.text5}>四象限</p>
            <div className={styles.backgroundShadow} />
          </div>
          <div className={styles.link3}>
            <img
              src="../image/mnfd9x5v-5z6ksvs.svg"
              className={styles.container6}
            />
            <p className={styles.text6}>Calendar</p>
          </div>
          <div className={styles.link4}>
            <img
              src="../image/mnfd9x5v-v1s6u9y.svg"
              className={styles.container7}
            />
            <p className={styles.text7}>Analytics</p>
          </div>
        </div>
        <div className={styles.button}>
          <div className={styles.buttonShadow}>
            <img
              src="../image/mnfd9x5v-b82778r.svg"
              className={styles.container8}
            />
            <p className={styles.text8}>New Task</p>
          </div>
        </div>
        <div className={styles.horizontalBorder}>
          <div className={styles.link5}>
            <img
              src="../image/mnfd9x5v-rk1pyjx.svg"
              className={styles.container9}
            />
            <p className={styles.text9}>Settings</p>
          </div>
          <div className={styles.link6}>
            <img
              src="../image/mnfd9x5v-zi4lflt.svg"
              className={styles.container4}
            />
            <p className={styles.text10}>Log Out</p>
          </div>
        </div>
      </div>
      <div className={styles.mainCanvas}>
        <div className={styles.header}>
          <div className={styles.container11}>
            <div className={styles.container10}>
              <p className={styles.text11}>Cognitive Productivity</p>
            </div>
            <p className={styles.text12}>艾森豪威尔矩阵</p>
          </div>
          <div className={styles.background2}>
            <div className={styles.button2}>
              <p className={styles.text13}>四象限视图</p>
            </div>
            <div className={styles.button3}>
              <p className={styles.text14}>列表视图</p>
            </div>
          </div>
        </div>
        <div className={styles.eisenhowerMatrixGrid}>
          <div className={styles.autoWrapper}>
            <div className={styles.sectionQuadrant1Impo}>
              <div className={styles.container14}>
                <div className={styles.container13}>
                  <div className={styles.container12}>
                    <p className={styles.text15}>Quadrant 1</p>
                  </div>
                  <p className={styles.text16}>重要且紧急</p>
                </div>
                <div className={styles.overlay}>
                  <p className={styles.text17}>立即处理 (DO)</p>
                </div>
              </div>
              <div className={styles.container17}>
                <div className={styles.taskCard}>
                  <p className={styles.text18}>完成 Q3 季度业务报告</p>
                  <p className={styles.text19}>
                    今天下午 5:00 前提交给董事会，包含所有核心数据指标。
                  </p>
                  <div className={styles.container15}>
                    <div className={styles.overlay2}>
                      <p className={styles.text20}>High Priority</p>
                    </div>
                    <div className={styles.background3}>
                      <p className={styles.text21}>Reporting</p>
                    </div>
                  </div>
                </div>
                <div className={styles.backgroundVerticalBo}>
                  <p className={styles.text18}>修复服务器生产环境崩溃</p>
                  <p className={styles.text19}>关键错误：核心数据库连接超时。</p>
                  <div className={styles.container16}>
                    <div className={styles.overlay3}>
                      <p className={styles.text22}>Emergency</p>
                    </div>
                  </div>
                </div>
              </div>
              <div className={styles.background4} />
            </div>
            <div className={styles.sectionQuadrant2Urge}>
              <div className={styles.container20}>
                <div className={styles.container19}>
                  <div className={styles.container18}>
                    <p className={styles.text23}>Quadrant 3</p>
                  </div>
                  <p className={styles.text24}>不重要但紧急</p>
                </div>
                <div className={styles.overlay4}>
                  <p className={styles.text25}>交由他人 (DELEGATE)</p>
                </div>
              </div>
              <div className={styles.container23}>
                <div className={styles.backgroundVerticalBo2}>
                  <p className={styles.text18}>回复日常运营邮件</p>
                  <p className={styles.text19}>
                    关于办公室用品申领和普通行政通知。
                  </p>
                  <div className={styles.container21}>
                    <div className={styles.overlay5}>
                      <p className={styles.text26}>Admin</p>
                    </div>
                  </div>
                </div>
                <div className={styles.backgroundVerticalBo3}>
                  <p className={styles.text18}>预定下周出差的酒店</p>
                  <p className={styles.text19}>北京出差，靠近国贸 CBD 区域。</p>
                  <div className={styles.container22}>
                    <div className={styles.background5}>
                      <p className={styles.text27}>Travel</p>
                    </div>
                  </div>
                </div>
              </div>
              <div className={styles.background6} />
            </div>
          </div>
          <div className={styles.autoWrapper2}>
            <div className={styles.sectionQuadrant4Impo}>
              <div className={styles.container26}>
                <div className={styles.container25}>
                  <div className={styles.container24}>
                    <p className={styles.text28}>Quadrant 2</p>
                  </div>
                  <p className={styles.text16}>重要不紧急</p>
                </div>
                <div className={styles.overlay6}>
                  <p className={styles.text29}>制定计划 (SCHEDULE)</p>
                </div>
              </div>
              <div className={styles.container29}>
                <div className={styles.backgroundVerticalBo4}>
                  <p className={styles.text18}>学习 Rust 编程语言</p>
                  <p className={styles.text19}>
                    每天早晨 1 小时的核心深度工作时间。
                  </p>
                  <div className={styles.container27}>
                    <div className={styles.overlay7}>
                      <p className={styles.text30}>Growth</p>
                    </div>
                  </div>
                </div>
                <div className={styles.backgroundVerticalBo5}>
                  <p className={styles.text18}>年度财务资产配置方案</p>
                  <p className={styles.text19}>回顾当前投资组合并进行平衡调整。</p>
                  <div className={styles.container28}>
                    <div className={styles.background7}>
                      <p className={styles.text31}>Finance</p>
                    </div>
                  </div>
                </div>
              </div>
              <div className={styles.background8} />
            </div>
            <div className={styles.sectionQuadrant3NotU}>
              <div className={styles.container32}>
                <div className={styles.container31}>
                  <div className={styles.container30}>
                    <p className={styles.text32}>Quadrant 4</p>
                  </div>
                  <p className={styles.text24}>不重要不紧急</p>
                </div>
                <div className={styles.overlay8}>
                  <p className={styles.text33}>尽量消除 (DELETE)</p>
                </div>
              </div>
              <div className={styles.background9} />
              <div className={styles.container34}>
                <div className={styles.backgroundVerticalBo6}>
                  <p className={styles.text18}>整理 2021 年的旧书签</p>
                  <p className={styles.text19}>目前不急需，可以等空闲时间处理。</p>
                  <div className={styles.container33}>
                    <div className={styles.overlay9}>
                      <p className={styles.text34}>Low Value</p>
                    </div>
                  </div>
                </div>
                <div className={styles.backgroundVerticalBo7}>
                  <p className={styles.text18}>社交媒体无目的浏览</p>
                  <p className={styles.text19}>
                    意识到这是时间黑洞，需要严格控制。
                  </p>
                </div>
              </div>
            </div>
          </div>
          <div className={styles.overlayBorderOverlay}>
            <div className={styles.overlayShadow2}>
              <img
                src="../image/mnfd9x5v-b9n3pgr.svg"
                className={styles.container35}
              />
              <p className={styles.text35}>Archive</p>
            </div>
          </div>
        </div>
        <div className={styles.footerStatusBar}>
          <div className={styles.container38}>
            <div className={styles.container36}>
              <div className={styles.backgroundShadow2} />
              <p className={styles.text36}>2 紧急任务</p>
            </div>
            <div className={styles.container37}>
              <div className={styles.backgroundShadow3} />
              <p className={styles.text37}>5 计划项</p>
            </div>
          </div>
          <div className={styles.container40}>
            <p className={styles.text38}>最后同步: 1 分钟前</p>
            <div className={styles.container39}>
              <div className={styles.avatar} />
              <div className={styles.margin}>
                <div className={styles.backgroundBorder}>
                  <p className={styles.text39}>+3</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default Component;
