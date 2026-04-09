import React from 'react';

import styles from './index.module.scss';

const Component = () => {
  return (
    <div className={styles.frame}>
      <div className={styles.mainContentArea}>
        <div className={styles.headerTopAppBar}>
          <div className={styles.container}>
            <p className={styles.text}>Schedule Overview</p>
            <div className={styles.heading2}>
              <p className={styles.text2}>2024年 3月</p>
            </div>
          </div>
          <div className={styles.container5}>
            <div className={styles.background}>
              <div className={styles.button}>
                <p className={styles.text3}>Day</p>
              </div>
              <div className={styles.button2}>
                <p className={styles.text4}>Week</p>
              </div>
              <div className={styles.button3}>
                <p className={styles.text5}>Month</p>
              </div>
            </div>
            <div className={styles.margin}>
              <div className={styles.verticalDivider} />
            </div>
            <div className={styles.container4}>
              <div className={styles.button4}>
                <img
                  src="../image/mnfcm55q-j3nmt9u.svg"
                  className={styles.container2}
                />
                <p className={styles.text6}>导出日报</p>
              </div>
              <div className={styles.button5}>
                <img
                  src="../image/mnfcm55q-mqyj03j.svg"
                  className={styles.container3}
                />
                <p className={styles.text6}>导出周报</p>
              </div>
            </div>
          </div>
        </div>
        <div className={styles.sectionCalendarGrid}>
          <div className={styles.weekdayHeaders}>
            <div className={styles.autoWrapper}>
              <div className={styles.container6}>
                <p className={styles.text7}>Sun</p>
              </div>
              <div className={styles.container7}>
                <p className={styles.text8}>Mon</p>
              </div>
            </div>
            <div className={styles.autoWrapper2}>
              <div className={styles.container8}>
                <p className={styles.text9}>Tue</p>
              </div>
              <div className={styles.container9}>
                <p className={styles.text10}>Wed</p>
              </div>
              <div className={styles.container10}>
                <p className={styles.text7}>Thu</p>
              </div>
              <div className={styles.container11}>
                <p className={styles.text11}>Fri</p>
              </div>
            </div>
            <div className={styles.container12}>
              <p className={styles.text9}>Sat</p>
            </div>
          </div>
          <div className={styles.calendarDays}>
            <div className={styles.autoWrapper5}>
              <div className={styles.autoWrapper3}>
                <div className={styles.emptySlotsForPrevMon}>
                  <p className={styles.text12}>25</p>
                </div>
                <div className={styles.overlay}>
                  <p className={styles.text12}>26</p>
                </div>
              </div>
              <div className={styles.autoWrapper4}>
                <div className={styles.emptySlotsForPrevMon}>
                  <p className={styles.text12}>27</p>
                </div>
                <div className={styles.overlay}>
                  <p className={styles.text12}>28</p>
                </div>
                <div className={styles.overlay2}>
                  <p className={styles.text12}>29</p>
                </div>
                <div className={styles.monthStart}>
                  <p className={styles.text13}>1</p>
                  <div className={styles.overlay3}>
                    <p className={styles.text14}>季度总结会议</p>
                  </div>
                </div>
              </div>
              <div className={styles.background2}>
                <p className={styles.text15}>2</p>
              </div>
            </div>
            <div className={styles.autoWrapper8}>
              <div className={styles.autoWrapper6}>
                <div className={styles.row2}>
                  <p className={styles.text15}>3</p>
                </div>
                <div className={styles.background3}>
                  <p className={styles.text15}>4</p>
                  <div className={styles.container13}>
                    <div className={styles.overlay4}>
                      <p className={styles.text16}>核心产品迭代</p>
                    </div>
                    <div className={styles.overlay5}>
                      <p className={styles.text17}>周报提交</p>
                    </div>
                  </div>
                </div>
              </div>
              <div className={styles.autoWrapper7}>
                <div className={styles.row2}>
                  <p className={styles.text15}>5</p>
                </div>
                <div className={styles.background4}>
                  <p className={styles.text15}>6</p>
                  <div className={styles.overlay6}>
                    <p className={styles.text18}>设计评审 (V2.0)</p>
                  </div>
                </div>
                <div className={styles.background5}>
                  <p className={styles.text19}>7</p>
                </div>
                <div className={styles.background6}>
                  <p className={styles.text15}>8</p>
                </div>
              </div>
              <div className={styles.background2}>
                <p className={styles.text15}>9</p>
              </div>
            </div>
            <div className={styles.autoWrapper12}>
              <div className={styles.autoWrapper9}>
                <div className={styles.row3TodayHighlight}>
                  <p className={styles.text20}>10</p>
                </div>
                <div className={styles.background7}>
                  <p className={styles.text21}>11</p>
                </div>
                <div className={styles.row4}>
                  <p className={styles.text22}>17</p>
                </div>
                <div className={styles.background8}>
                  <p className={styles.text20}>18</p>
                  <div className={styles.overlay7}>
                    <p className={styles.text23}>全员大会</p>
                  </div>
                </div>
              </div>
              <div className={styles.autoWrapper10}>
                <div className={styles.row3TodayHighlight}>
                  <p className={styles.text20}>13</p>
                </div>
                <div className={styles.background9}>
                  <p className={styles.text24}>14</p>
                </div>
                <div className={styles.background10}>
                  <p className={styles.text20}>15</p>
                  <div className={styles.overlay5}>
                    <p className={styles.text17}>项目复盘</p>
                  </div>
                </div>
                <div className={styles.background11}>
                  <p className={styles.text20}>19</p>
                </div>
                <div className={styles.background12}>
                  <p className={styles.text25}>20</p>
                </div>
                <div className={styles.background13}>
                  <p className={styles.text20}>21</p>
                  <div className={styles.overlay8}>
                    <p className={styles.text26}>技术预研</p>
                  </div>
                </div>
                <div className={styles.background14}>
                  <p className={styles.text25}>22</p>
                </div>
                <div className={styles.aCtiveday}>
                  <div className={styles.background15}>
                    <p className={styles.text27}>12</p>
                  </div>
                  <div className={styles.container15}>
                    <div className={styles.backgroundShadow}>
                      <p className={styles.text28}>今日重点：SAN…</p>
                    </div>
                    <div className={styles.overlay4}>
                      <p className={styles.text16}>客户回访任务</p>
                    </div>
                    <div className={styles.container14}>
                      <div className={styles.background16} />
                      <p className={styles.text29}>Active Now</p>
                    </div>
                  </div>
                  <div className={styles.background17} />
                </div>
              </div>
              <div className={styles.autoWrapper11}>
                <div className={styles.row3TodayHighlight}>
                  <p className={styles.text20}>16</p>
                </div>
                <div className={styles.background12}>
                  <p className={styles.text25}>23</p>
                </div>
              </div>
            </div>
            <div className={styles.autoWrapper15}>
              <div className={styles.autoWrapper13}>
                <div className={styles.row5}>
                  <p className={styles.text25}>24</p>
                </div>
                <div className={styles.background18}>
                  <p className={styles.text25}>25</p>
                </div>
              </div>
              <div className={styles.autoWrapper14}>
                <div className={styles.row5}>
                  <p className={styles.text25}>26</p>
                </div>
                <div className={styles.background19}>
                  <p className={styles.text30}>27</p>
                </div>
                <div className={styles.background20}>
                  <p className={styles.text25}>28</p>
                </div>
                <div className={styles.background21}>
                  <p className={styles.text25}>29</p>
                </div>
              </div>
              <div className={styles.background22}>
                <p className={styles.text25}>30</p>
              </div>
            </div>
          </div>
        </div>
        <div className={styles.insightsBentoSection}>
          <div className={styles.backgroundShadow2}>
            <div className={styles.container17}>
              <p className={styles.text31}>本月概览</p>
              <img
                src="../image/mnfcm55r-f0rtpbu.svg"
                className={styles.container16}
              />
            </div>
            <div className={styles.container19}>
              <div className={styles.background23}>
                <p className={styles.text32}>已完成任务</p>
                <div className={styles.paragraph}>
                  <p className={styles.text33}>24&nbsp;</p>
                  <p className={styles.text34}>/ 30</p>
                </div>
              </div>
              <div className={styles.background24}>
                <p className={styles.text32}>专注时长</p>
                <p className={styles.a142H}>142h</p>
              </div>
              <div className={styles.background26}>
                <p className={styles.text32}>效率趋势</p>
                <div className={styles.container18}>
                  <div className={styles.overlay9} />
                  <div className={styles.overlay10} />
                  <div className={styles.overlay11} />
                  <div className={styles.background25} />
                </div>
              </div>
            </div>
          </div>
          <div className={styles.background27}>
            <div className={styles.overlayBlur} />
            <img
              src="../image/mnfcm55z-kgef1dl.png"
              className={styles.zenWorkspace}
            />
            <div className={styles.container21}>
              <p className={styles.text35}>认知避难所</p>
              <div className={styles.container20}>
                <p className={styles.text36}>
                  专注力是现代社会最稀缺的资源。在
                  <br />
                  这个月，你已经在日历中规划了 8<br />
                  个深读时段。
                </p>
              </div>
              <div className={styles.button6}>
                <p className={styles.text37}>开始深读</p>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div className={styles.asideSidebarNavigati}>
        <div className={styles.container25}>
          <div className={styles.background28}>
            <div className={styles.overlayShadow}>
              <img
                src="../image/mnfcm55r-7f7uqzw.svg"
                className={styles.container22}
              />
            </div>
          </div>
          <div className={styles.container24}>
            <p className={styles.text38}>Sanctuary</p>
            <div className={styles.container23}>
              <p className={styles.text39}>Deep Work Mode</p>
            </div>
          </div>
        </div>
        <div className={styles.button7}>
          <div className={styles.buttonShadow}>
            <img
              src="../image/mnfcm55r-cz7k8na.svg"
              className={styles.container26}
            />
            <p className={styles.text40}>New Task</p>
          </div>
        </div>
        <div className={styles.nav}>
          <div className={styles.link}>
            <img
              src="../image/mnfcm55r-rotfu5m.svg"
              className={styles.container27}
            />
            <p className={styles.text41}>Dashboard</p>
          </div>
          <div className={styles.link2}>
            <img
              src="../image/mnfcm55r-u2zabvk.svg"
              className={styles.container28}
            />
            <p className={styles.text42}>Tasks</p>
          </div>
          <div className={styles.link3}>
            <img
              src="../image/mnfcm55r-eoh0btm.svg"
              className={styles.container29}
            />
            <p className={styles.text43}>Calendar</p>
            <div className={styles.backgroundShadow3} />
          </div>
          <div className={styles.link4}>
            <img
              src="../image/mnfcm55r-mt71pml.svg"
              className={styles.container28}
            />
            <p className={styles.text44}>Archive</p>
          </div>
          <div className={styles.link5}>
            <img
              src="../image/mnfcm55r-gl4oxvl.svg"
              className={styles.container30}
            />
            <p className={styles.text45}>Analytics</p>
          </div>
        </div>
        <div className={styles.horizontalBorder}>
          <div className={styles.link6}>
            <img
              src="../image/mnfcm55r-hxu2ntl.svg"
              className={styles.container28}
            />
            <p className={styles.text46}>Settings</p>
          </div>
          <div className={styles.link7}>
            <img
              src="../image/mnfcm55r-igdfz9x.svg"
              className={styles.container27}
            />
            <p className={styles.text47}>Log Out</p>
          </div>
        </div>
      </div>
    </div>
  );
}

export default Component;
