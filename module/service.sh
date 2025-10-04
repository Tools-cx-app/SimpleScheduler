#!/system/bin/sh
MODDIR=${0%/*}

until [ -d $MODDIR ]; do
  sleep 1
done

killall fas-rs-next
RUST_BACKTRACE=1 nohup $MODDIR/SimpleScheduler $MODDIR/config.toml >$LOG 2>&1 &
