#!/system/bin/sh
MODDIR=${0%/*}
LOG=$MODDIR/run.log

until [ -d $MODDIR ]; do
  sleep 1
done

RUST_BACKTRACE=1 nohup $MODDIR/SimpleScheduler $MODDIR/config.toml >$LOG 2>&1 &
