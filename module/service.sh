#!/system/bin/sh
MODDIR=${0%/*}
LOG=$MODDIR/run.log

until [ -d $MODDIR ]; do
  sleep 1
done

RUST_BACKTRACE=1 nohup $MODDIR/SimpleScheduler /data/adb/SimpleScheduler/config.toml >$LOG 2>&1 &
