import React from 'react';

import styles from './index.module.scss';

const Component = () => {
  return (
    <div className={styles.asideSideNavBarShare}>
      <div className={styles.container4}>
        <div className={styles.background}>
          <div className={styles.overlayShadow}>
            <img src="../image/mnfdqhzj-vn9861t.svg" className={styles.container} />
          </div>
        </div>
        <div className={styles.container3}>
          <p className={styles.text}>Sanctuary</p>
          <div className={styles.container2}>
            <p className={styles.text2}>Deep Work Mode</p>
          </div>
        </div>
      </div>
      <div className={styles.nav}>
        <div className={styles.link}>
          <img src="../image/mnfdqhzj-xkakd50.svg" className={styles.container5} />
          <p className={styles.text3}>Dashboard</p>
        </div>
        <div className={styles.link2}>
          <img src="../image/mnfdqhzj-wn4hq7g.svg" className={styles.container6} />
          <p className={styles.text4}>Tasks</p>
        </div>
        <div className={styles.link3}>
          <img src="../image/mnfdqhzj-k4hucq9.svg" className={styles.container6} />
          <p className={styles.text5}>标签管理</p>
          <div className={styles.backgroundShadow} />
        </div>
        <div className={styles.link4}>
          <img src="../image/mnfdqhzj-zvn5bl6.svg" className={styles.container7} />
          <p className={styles.text6}>Calendar</p>
        </div>
        <div className={styles.link5}>
          <img src="../image/mnfdqhzj-r6wlkza.svg" className={styles.container8} />
          <p className={styles.text7}>Analytics</p>
        </div>
      </div>
      <div className={styles.margin}>
        <div className={styles.button}>
          <div className={styles.buttonShadow}>
            <img
              src="../image/mnfdqhzj-bs0lgyg.svg"
              className={styles.container9}
            />
            <p className={styles.text8}>New Task</p>
          </div>
        </div>
      </div>
      <div className={styles.container10}>
        <div className={styles.link6}>
          <img src="../image/mnfdqhzj-jgmrocw.svg" className={styles.container6} />
          <p className={styles.text9}>Settings</p>
        </div>
        <div className={styles.link7}>
          <img src="../image/mnfdqhzj-29vnfmm.svg" className={styles.container5} />
          <p className={styles.text10}>Log Out</p>
        </div>
      </div>
    </div>
  );
}

export default Component;
