import React from 'react';

import styles from './index.module.scss';

const Component = () => {
  return (
    <div className={styles.headerTopNavBarShare}>
      <div className={styles.container3}>
        <p className={styles.text}>The Cognitive Sanctuary</p>
        <div className={styles.input}>
          <div className={styles.container}>
            <p className={styles.text2}>搜索标签...</p>
          </div>
          <img src="../image/mnfdqhzg-mqu19kp.svg" className={styles.container2} />
        </div>
      </div>
      <div className={styles.container6}>
        <div className={styles.button}>
          <img src="../image/mnfdqhzg-gvhqnry.svg" className={styles.container4} />
        </div>
        <div className={styles.button2}>
          <img src="../image/mnfdqhzg-v79nv6h.svg" className={styles.container5} />
        </div>
        <div className={styles.button2}>
          <img src="../image/mnfdqhzg-whpk3rq.svg" className={styles.container5} />
        </div>
        <div className={styles.margin}>
          <div className={styles.overlayShadow}>
            <img
              src="../image/mnfdqhzg-aq8yp1l.png"
              className={styles.userAvatar}
            />
          </div>
        </div>
      </div>
    </div>
  );
}

export default Component;
