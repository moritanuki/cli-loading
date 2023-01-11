use chrono::prelude::*;

fn main () {
    let loading_markers = vec!["|", "/", "-", "\\"];
    let mut progress = String::from("====================");  // 100%状態の文字
  
    let endtime = Local.datetime_from_str("2023-01-12 08:32:00", "%Y-%m-%d %H:%M:%S")
      .unwrap()
      .timestamp_millis();
    let range = endtime - Local::now().timestamp_millis();
    
    loop {
      let diff = endtime - Local::now().timestamp_millis();
      let marker = loading_markers[(diff / 100) as usize % loading_markers.len()];
      print!("{} [{}]\r", marker, progress);
  
      if diff <= 0 {
        break;
      } else {
        let percent = (diff as f64 / range as f64) * 100.0;
        let index = (20.0 * (percent / 100.0) - 1.0) as i64;
        // percentに見合うindexを "<"に置き換える
        progress.remove(index as usize);
        progress.insert(index as usize, '<');
        if (index + 1) < 20 {
          // 減ってなくなったところは空文字にする
          progress.remove((index + 1) as usize);
          progress.insert((index + 1) as usize, ' ');
        }
      }
    }
  }